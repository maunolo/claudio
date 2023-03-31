use anyhow::Result;
use clap::{Args, Subcommand};

mod emitter;
mod receptor;

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

    /// The output audio device name to use or "default",
    /// you can use the "list-devices" command
    /// to list all available output device names
    /// (e.g. "default")
    #[arg(short = 'd', default_value = "default")]
    device: String,
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
}

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Receptor(args) => receptor::receptor(args)?,
        Commands::Emitter(args) => emitter::emitter(args)?,
    }

    Ok(())
}
