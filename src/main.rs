mod scan_libinput;

use rodio::{OutputStream, OutputStreamBuilder};

use std::{fs::File, sync::Arc};

const OPT_PATH_AUDIO: &str = "wav";

fn play(code: u32, press: u32, stream_handle: Arc<OutputStream>) {
    let file_name = format!("{OPT_PATH_AUDIO}/{code:02}-{press:?}.wav");
    if let Ok(file) = File::open(file_name) {
        std::thread::spawn(move || {
            let sink = rodio::play(&stream_handle.mixer(), file).expect("Failed to create sink");
            sink.sleep_until_end();
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    scan_libinput::handle_key(Arc::new(stream_handle))?;

    Ok(())
}
