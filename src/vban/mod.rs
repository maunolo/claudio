use anyhow::Result;
use clap::{Args, Subcommand};

use rusty_vban::{emitter, receptor};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Receive audio from a VBAN stream
    Receptor(ReceptorArgs),
    /// Send audio to a VBAN stream
    Emitter(EmitterArgs),
}

#[derive(Args, Debug)]
pub struct ReceptorArgs {
    /// The latency in milliseconds from the input stream
    #[arg(short = 'l', default_value = "16")]
    latency: u32,

    /// The name of the stream to listen to
    #[arg(short = 's', required = true)]
    stream_name: String,

    /// The number of channels to listen to
    /// (1 for mono, 2 for stereo, etc.)
    #[arg(short = 'c', default_value = "2")]
    channels: u8,

    /// The IP address of the VBAN emitter
    /// (e.g. 192.168.0.1)
    #[arg(short = 'i', required = true)]
    ip_address: String,

    /// The port of this VBAN receptor
    /// (e.g. 6980)
    #[arg(short = 'p', default_value = "6980")]
    port: u16,

    /// The audio device name to use or "default",
    /// you can use the "list-devices" command
    /// to list all available device names
    /// (e.g. "default")
    #[arg(short = 'd', default_value = "default")]
    device: String,

    /// The audio device type to use
    /// depending on the OS the device names can be the same
    /// for both input and output devices so you can use this
    /// (e.g. "output")
    #[arg(short = 't', default_value = "output")]
    device_type: String,

    /// The audio backend to use: ALSA, JACK
    /// (e.g. "default")
    #[arg(short = 'b', default_value = "default")]
    backend: String,
}

#[derive(Args, Debug)]
pub struct EmitterArgs {
    /// The name of the stream to listen to
    #[arg(short = 's', required = true)]
    stream_name: String,

    /// The number of channels to listen to
    /// (1 for mono, 2 for stereo, etc.)
    #[arg(short = 'c', default_value = "2")]
    channels: u8,

    /// The IP address of the VBAN receiver
    /// (e.g. 192.168.0.1)
    #[arg(short = 'i', required = true)]
    ip_address: String,

    /// The port of the VBAN receiver
    /// (e.g. 6980)
    #[arg(short = 'p', default_value = "6980")]
    port: u16,

    /// The audio device name to use or "default",
    /// you can use the "list-devices" command
    /// to list all available device names
    /// (e.g. "default")
    #[arg(short = 'd', default_value = "default")]
    device: String,

    /// The audio device type to use
    /// depending on the OS the device names can be the same
    /// for both input and output devices so you can use this
    /// (e.g. "input")
    #[arg(short = 't', default_value = "input")]
    device_type: String,

    /// The audio backend to use: ALSA, JACK
    /// (e.g. "default")
    #[arg(short = 'b', default_value = "default")]
    backend: String,
}

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Receptor(args) => {
            receptor::ReceptorBuilder::default()
                .ip_address(args.ip_address)
                .port(args.port)
                .stream_name(args.stream_name)
                .channels(args.channels)
                .latency(args.latency)
                .device(args.device)
                .device_type(args.device_type)
                .backend(args.backend)
                .build()?
                .start()?;
        }
        Commands::Emitter(args) => {
            emitter::EmitterBuilder::default()
                .ip_address(args.ip_address)
                .port(args.port)
                .stream_name(args.stream_name)
                .channels(args.channels)
                .device(args.device)
                .device_type(args.device_type)
                .backend(args.backend)
                .build()?
                .start()?;
        }
    }

    Ok(())
}
