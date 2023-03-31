mod stream;

use anyhow::Result;

use self::stream::VbanEmitterStream;

pub fn emitter(args: crate::vban::EmitterArgs) -> Result<()> {
    let mut stream = VbanEmitterStream::new(&args)?;
    stream.setup_stream()?;
    stream.play()?;

    while stream.should_run(&args) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    stream.pause()?;
    drop(stream);

    emitter(args)?;

    Ok(())
}
