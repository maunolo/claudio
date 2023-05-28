// #![windows_subsystem = "windows"] // Hide console window on Windows

mod tools;
mod vban;

use clap::{Args, Parser, Subcommand};

use tools::{default_input, default_output, list_backends, list_devices};

#[derive(Parser, Debug)]
#[command(
    author = "Maunolo",
    version = "0.3.0",
    about = "CLI audio tool (Claudio)",
    long_about = "
This tool allows you to send and receive audio over a network using the VBAN protocol.
VBAN is a protocol for sending audio over a network.

This tool is a work in progress. It is not yet feature complete and may not work as expected.
Please report any issues you encounter on the GitHub page.

Only supports 48kHz, PCM 16-bits, 2-channels audio for now. This will be expanded in the future.

This tool is not affiliated with VBAN.
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Send and Receive audio from a VBAN stream
    Vban {
        #[command(subcommand)]
        command: vban::Commands,
    },
    /// List audio devices
    ListDevices(BackendArgs),
    /// List audio hosts
    ListBackends,
    /// Default output device
    DefaultOutput(BackendArgs),
    /// Default input device
    DefaultInput(BackendArgs),
}

#[derive(Args, Debug)]
pub struct BackendArgs {
    #[arg(short = 'b', default_value = "default")]
    backend: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Vban { command } => {
            if let Err(e) = vban::run(command) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::ListDevices(args) => {
            if let Err(e) = list_devices(Some(&args.backend)) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::DefaultOutput(args) => {
            if let Err(e) = default_output(Some(&args.backend)) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::DefaultInput(args) => {
            if let Err(e) = default_input(Some(&args.backend)) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::ListBackends => {
            if let Err(e) = list_backends() {
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
