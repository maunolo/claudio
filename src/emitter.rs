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
    let mut buffer = [0; vban::MAX_PACKET_SIZE];

    header.set_num_samples(input.len() as u8);
    header.set_frame_number(*frame_number);
    let header: [u8; 28] = header.into();
    let data = input
        .iter()
        .flat_map(|s| s.to_sample::<i16>().to_le_bytes())
        .collect::<Vec<u8>>();
    for (index, byte) in header.iter().enumerate() {
        buffer[index] = *byte;
    }
    for (index, byte) in data.iter().enumerate() {
        let i = index + 28;
        if i >= vban::MAX_PACKET_SIZE - 1 {
            break;
        }
        buffer[i] = *byte;
    }
    socket.send_to(&buffer, addr).unwrap();

    *frame_number += 1;
}

pub fn emitter(_args: super::EmitterArgs) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or(anyhow!("no input device available"))?;
    let config = device.default_input_config()?;

    let addrs = [
        SocketAddr::from(([0, 0, 0, 0], 6981)),
        SocketAddr::from(([0, 0, 0, 0], 6982)),
        SocketAddr::from(([0, 0, 0, 0], 6983)),
        SocketAddr::from(([0, 0, 0, 0], 6984)),
        SocketAddr::from(([0, 0, 0, 0], 6985)),
        SocketAddr::from(([0, 0, 0, 0], 6986)),
        SocketAddr::from(([0, 0, 0, 0], 6987)),
        SocketAddr::from(([0, 0, 0, 0], 6988)),
        SocketAddr::from(([0, 0, 0, 0], 6989)),
        SocketAddr::from(([0, 0, 0, 0], 6990)),
    ];

    let socket = UdpSocket::bind(&addrs[..])?;
    let header = vban::Header::new("Mic");
    let addr = SocketAddr::from(([192, 168, 50, 4], 6980));
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
