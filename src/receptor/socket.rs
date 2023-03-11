use anyhow::{anyhow, Context, Result};

use std::net::{IpAddr, SocketAddr, UdpSocket};

use byteorder::{ByteOrder, LittleEndian};

use super::ring_buffer::VbanStreamProducer;

pub struct VbanReceptorSocket {
    socket: UdpSocket,
    buf: [u8; vban::MAX_PACKET_SIZE],
}

impl VbanReceptorSocket {
    pub fn new(args: &crate::ReceptorArgs) -> Result<Self> {
        let addr = SocketAddr::new("0.0.0.0".parse()?, args.port);
        let socket = UdpSocket::bind(addr)?;

        Ok(VbanReceptorSocket {
            socket,
            buf: [0; vban::MAX_PACKET_SIZE],
        })
    }

    pub fn start_receive_loop<F>(
        mut self,
        args: &crate::ReceptorArgs,
        mut producer: VbanStreamProducer,
        should_run_callback: F,
    ) -> Result<()>
    where
        F: Fn() -> bool,
    {
        while should_run_callback() {
            match self.receive_packet(&args) {
                Ok(pkt) => {
                    for sample in pkt.data.chunks_exact(2) {
                        let sample = LittleEndian::read_i16(&sample);
                        producer.push(sample).ok();
                    }
                }
                Err(e) => println!("Warning: {}", e),
            }
        }

        Ok(())
    }

    fn receive_packet(&mut self, args: &crate::ReceptorArgs) -> Result<vban::Packet> {
        let (amt, src) = self.socket.recv_from(&mut self.buf).context(format!(
            "Failed to receive packet from socket: {:?}",
            self.socket
        ))?;

        check_src(&args, &src)?;

        let packet = vban::Packet::try_from(&self.buf[..amt])?;

        check_audio_pkt(&args, &packet)?;

        Ok(packet)
    }
}

fn check_src(args: &crate::ReceptorArgs, src: &SocketAddr) -> Result<()> {
    if src.ip()
        != args
            .ip_address
            .parse::<IpAddr>()
            .context("Informed IP address is invalid")?
    {
        return Err(anyhow!("Wrong source"));
    }

    Ok(())
}

fn check_audio_pkt(args: &crate::ReceptorArgs, pkt: &vban::Packet) -> Result<()> {
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
