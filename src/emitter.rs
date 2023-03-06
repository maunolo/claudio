use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample,
};
use dasp_sample::conv::ToSample;
use std::net::{SocketAddr, UdpSocket};

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

fn write_data<T>(
    input: &[T],
    mut header: vban::Header,
    socket: &UdpSocket,
    addr: SocketAddr,
    frame_number: &mut u32,
) where
    T: Sample + ToSample<i16>,
{
    for samples in input.chunks_exact(240) {
        let mut buffer = [0; 240 * 2 + 28];

        header.set_num_samples((samples.len() / 2) as u8);
        header.set_frame_number(*frame_number);
        let header: [u8; 28] = header.into();
        let data = samples
            .iter()
            .flat_map(|s| s.to_sample::<i16>().to_le_bytes())
            .collect::<Vec<u8>>();
        for (index, byte) in header.iter().enumerate() {
            buffer[index] = *byte;
        }
        for (index, byte) in data.iter().enumerate() {
            buffer[index + 28] = *byte;
        }
        socket.send_to(&buffer, addr).unwrap();

        *frame_number += 1;
    }
}

pub fn emitter(args: super::EmitterArgs) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or(anyhow!("no input device available"))?;
    let config = device.default_input_config()?;

    let addrs = (1..=10)
        .map(|i| SocketAddr::from(([0, 0, 0, 0], 6980 + i)))
        .collect::<Vec<SocketAddr>>();

    let socket = UdpSocket::bind(&addrs[..])?;
    let header = vban::Header::new(&args.stream_name);
    let addr = SocketAddr::new(args.ip_address.parse()?, args.port);
    let mut frame_number: u32 = 0;

    println!("default input device: {}", device.name()?);
    println!("sample rate: {}", config.sample_rate().0);
    println!("channels: {}", config.channels());
    println!("sample format: {:?}", config.sample_format());

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_data::<f32>(data, header, &socket, addr, &mut frame_number),
            err_fn,
            None,
        )?,
        _ => panic!("Unsupported sample format"),
    };

    stream.play()?;

    std::thread::park();

    Ok(())
}
