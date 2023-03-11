use anyhow::{anyhow, Result};
use dasp_sample::ToSample;

use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Sample, SampleFormat, SizedSample, SupportedStreamConfig,
};

use crate::utils::cpal::{Device, Host};

pub struct VbanEmitterStream {
    host: cpal::Host,
    device: Arc<cpal::Device>,
    stream: Option<cpal::Stream>,
    socket: Arc<UdpSocket>,
    header: vban::Header,
    target_address: Arc<SocketAddr>,
}

impl VbanEmitterStream {
    pub fn new(args: &crate::EmitterArgs) -> Result<Self> {
        let host = cpal::default_host();
        let device = Arc::new(
            host.find_input_device(&args.device)
                .ok_or(anyhow!("no input device available"))?,
        );
        let addrs = (1..=10)
            .map(|i| SocketAddr::from(([0, 0, 0, 0], 6980 + i)))
            .collect::<Vec<SocketAddr>>();

        let socket = Arc::new(UdpSocket::bind(&addrs[..])?);
        let header = vban::Header::new(&args.stream_name);
        let target_address = Arc::new(SocketAddr::new(args.ip_address.parse()?, args.port));

        Ok(Self {
            host,
            device,
            stream: None,
            socket,
            header,
            target_address,
        })
    }

    pub fn setup_stream(&mut self) -> Result<()> {
        let sample_format = self.device_config()?.sample_format();
        self.stream = Some(self.build_stream_for_sample_format(sample_format)?);

        Ok(())
    }

    pub fn device_config(&self) -> Result<SupportedStreamConfig> {
        Ok(self.device.default_config()?)
    }

    pub fn play(&self) -> Result<()> {
        self.stream()?.play()?;

        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        self.stream()?.pause()?;

        Ok(())
    }

    pub fn stream(&self) -> Result<&cpal::Stream> {
        let error_fn = || anyhow!("you first need to call .setup_stream in the VbanEmitterStream");

        Ok(self.stream.as_ref().ok_or_else(error_fn)?)
    }

    pub fn should_run(&self, args: &crate::EmitterArgs) -> bool {
        if args.device == "default" && !self.device.is_default_input(&self.host) {
            self.pause().ok();

            return false;
        }

        true
    }

    fn build_stream_for_sample_format(&self, sample_format: SampleFormat) -> Result<cpal::Stream> {
        let sample_format = sample_format;

        let stream = match sample_format {
            SampleFormat::F32 => self.build_stream::<f32>()?,
            SampleFormat::I16 => self.build_stream::<i16>()?,
            SampleFormat::U16 => self.build_stream::<u16>()?,
            _ => unreachable!("unsupported sample format: {:?}", sample_format),
        };

        Ok(stream)
    }

    fn build_stream<T>(&self) -> Result<cpal::Stream>
    where
        T: SizedSample + ToSample<i16> + Send + 'static,
    {
        let err_fn = move |error| eprintln!("an error occurred on stream: {}", error);
        let mut frame_count = 0;

        let header = self.header.clone();
        let socket = self.socket.clone();
        let target_address = self.target_address.clone();

        let stream = self.device.build_input_stream(
            &self.device_config()?.into(),
            move |data: &[T], _: &_| {
                write_data::<T>(data, header, &socket, &target_address, &mut frame_count)
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }
}

fn write_data<T>(
    input: &[T],
    mut header: vban::Header,
    socket: &UdpSocket,
    addr: &SocketAddr,
    frame_count: &mut u32,
) where
    T: Sample + ToSample<i16>,
{
    let total_samples = input.len() as usize;
    let max = vban::header::MAX_NUM_SAMPLES as f32;
    let chunks_amount = (total_samples as f32 / max).ceil() as usize;
    let chunk_num_samples = total_samples / chunks_amount;

    for samples in input.chunks_exact(chunk_num_samples) {
        let mut buffer = Vec::new();

        header.set_num_samples(samples.len() as u8 / header.num_channels());
        header.set_frame_number(*frame_count);
        let header: [u8; 28] = header.into();
        let data = samples
            .iter()
            .flat_map(|s| s.to_sample::<i16>().to_le_bytes())
            .collect::<Vec<u8>>();

        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&data);
        if let Err(e) = socket.send_to(&buffer[..buffer.len()], addr) {
            eprintln!("error sending data: {}", e);
        }

        *frame_count += 1;
    }
}
