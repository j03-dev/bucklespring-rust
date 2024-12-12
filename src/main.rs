mod scan_libinput;

use anyhow::Result;
use rodio::{OutputStream, OutputStreamHandle};
use std::{fs::File, sync::Arc};

const OPT_PATH_AUDIO: &str = "wav";

fn play(key_code: u32, key_state: u32, stream_handle: Arc<OutputStreamHandle>) -> Result<()> {
    let file_name = format!("{OPT_PATH_AUDIO}/{key_code:02}-{key_state:?}.wav");
    if let Ok(file) = File::open(file_name) {
        std::thread::spawn(move || {
            let sink = stream_handle
                .play_once(file)
                .expect("Failed to create sink");
            sink.sleep_until_end();
        });
    }

    Ok(())
}

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let stream_handle = Arc::new(stream_handle);
    scan_libinput::handle_key_press(stream_handle)?;

    Ok(())
}
