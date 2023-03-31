mod ring_buffer;
mod socket;
mod stream;

use anyhow::Result;

use self::{socket::VbanReceptorSocket, stream::VbanReceptorStream};

pub fn receptor(args: crate::vban::ReceptorArgs) -> Result<()> {
    let mut stream = VbanReceptorStream::new(&args)?;

    let (producer, consumer) = ring_buffer::start_buffer(&args, &stream.device_config()?);

    stream.setup_stream(consumer)?;
    stream.play()?;

    let socket = VbanReceptorSocket::new(&args)?;
    socket.start_receive_loop(&args, producer, || stream.should_run(&args));

    stream.pause()?;
    drop(stream);

    receptor(args)?;

    Ok(())
}
