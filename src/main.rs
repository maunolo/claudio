mod emitter;
mod list_devices;
mod receptor;
mod utils;

use clap::{Args, Parser, Subcommand};

use emitter::emitter;
use list_devices::list_devices;
use receptor::receptor;

#[derive(Parser, Debug)]
#[command(author = "Maunolo", version = "0.1.0", about = "VBAN audio command line tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Receive audio from a VBAN stream
    Receptor(ReceptorArgs),
    /// Send audio to a VBAN stream
    Emitter(EmitterArgs),
    /// List audio devices
    ListDevices,
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Receptor(args) => {
            if let Err(e) = receptor(args) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Emitter(args) => {
            if let Err(e) = emitter(args) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::ListDevices => {
            if let Err(e) = list_devices() {
                eprintln!("Error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli() {}
}
