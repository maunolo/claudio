mod emitter;
mod receptor;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use emitter::emitter;
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
}

#[derive(Args, Debug)]
pub struct ReceptorArgs {
    /// The latency in milliseconds from the input stream
    #[arg(short, long, default_value = "16")]
    latency: u32,
}

#[derive(Args, Debug)]
pub struct EmitterArgs {
    /// The latency in milliseconds from the output stream
    #[arg(short, long, default_value = "16")]
    latency: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Receptor(args) => {
            receptor(args).ok();
        }
        Commands::Emitter(args) => {
            emitter(args).ok();
        }
    }

    Ok(())
}
