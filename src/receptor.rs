use super::utils::socket_addrs_from_args;
use anyhow::{anyhow, Result};
use std::net::{IpAddr, UdpSocket};
use std::sync::Arc;
use std::{mem::MaybeUninit, net::SocketAddr};

use byteorder::{ByteOrder, LittleEndian};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};

use ringbuf::{Consumer, HeapRb, SharedRb};

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

fn write_data<T, I>(
    output: &mut [T],
    channels: usize,
    consumer: &mut Consumer<I, Arc<SharedRb<I, Vec<MaybeUninit<I>>>>>,
) where
    T: Sample + FromSample<I> + FromSample<f32>,
    I: Sample,
{
    let mut input_fell_behind = false;
    for frame in output.chunks_mut(channels) {
        for sample in frame {
            *sample = match consumer.pop() {
                Some(s) => s.to_sample::<T>(),
                None => {
                    input_fell_behind = true;
                    0.0.to_sample::<T>()
                }
            };
        }
    }
    if input_fell_behind {
        eprintln!("input stream fell behind: try increasing latency");
    }
}

pub fn run<T, I>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut consumer: Consumer<I, Arc<SharedRb<I, Vec<MaybeUninit<I>>>>>,
) -> Result<()>
where
    T: SizedSample + FromSample<I> + FromSample<f32>,
    I: Sample + std::marker::Send + 'static,
{
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut consumer)
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    std::thread::park();

    Ok(())
}

fn check_src(args: &super::ReceptorArgs, src: &SocketAddr) -> Result<()> {
    if src.ip() != args.ip_address.parse::<IpAddr>()? {
        return Err(anyhow!("Wrong source"));
    }

    Ok(())
}

fn check_audio_pkt(args: &super::ReceptorArgs, pkt: &vban::Packet) -> Result<()> {
    let header = pkt.header();

    // Check stream name
    if header.stream_name() != args.stream_name {
        return Err(anyhow!("Wrong stream name"));
    }

    if header.num_channels() != args.channels {
        return Err(anyhow!("Wrong number of channels"));
    }

    if !matches!(header.sub_protocol(), vban::SubProtocol::Audio) {
        return Err(anyhow!("Wrong sub protocol"));
    }

    if !matches!(header.codec(), vban::Codec::PCM) {
        return Err(anyhow!("Wrong codec"));
    }

    Ok(())
}

pub fn receptor(args: super::ReceptorArgs) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(anyhow!("no output device available"))?;
    let config = device.default_output_config()?;

    let latency_frames = (args.latency as f32 / 1_000.0) * config.sample_rate().0 as f32;
    let latency_samples = latency_frames as usize * config.channels() as usize;

    let ring = HeapRb::<i16>::new(latency_samples * 2);
    let (mut producer, consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0.to_sample::<i16>()).ok();
    }

    std::thread::spawn(move || -> Result<()> {
        match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32, i16>(&device, &config.into(), consumer)?,
            _ => panic!("Unsupported sample format"),
        };

        Ok(())
    });

    let addr = socket_addrs_from_args("0.0.0.0", args.port)?;
    let socket = UdpSocket::bind(addr)?;

    loop {
        let mut buf = [0; vban::MAX_PACKET_SIZE];
        let result = socket.recv_from(&mut buf);
        match result {
            Ok((amt, src)) => {
                // Parse packet
                let pkt = vban::Packet::try_from(&buf[..amt]);

                match pkt {
                    Ok(pkt) => {
                        if let Ok(_) = check_src(&args, &src) {
                            // Check packet
                            check_audio_pkt(&args, &pkt)?;

                            for sample in pkt.data.chunks_exact(2) {
                                let sample = LittleEndian::read_i16(&sample);
                                producer.push(sample).ok();
                            }
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            Err(e) => println!("Error: {:?}", e),
        };
    }
}
