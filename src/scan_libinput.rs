use anyhow::Result;

use input::event::keyboard::{KeyState, KeyboardEventTrait};
use input::{Event, Libinput, LibinputInterface};

use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use rodio::OutputStreamHandle;

use std::fs::{File, OpenOptions};
use std::os::fd::OwnedFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::Arc;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        let f = 0; // false
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != f) | (flags & O_RDWR != f))
            .write((flags & O_WRONLY != f) | (flags & O_RDWR != f))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

pub fn handle_key(stream_handle: Arc<OutputStreamHandle>) -> Result<()> {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            if let Event::Keyboard(key) = event {
                let code = key.key();
                let state = match key.key_state() {
                    KeyState::Pressed => 1,
                    KeyState::Released => 0,
                };
                super::play(code, state, stream_handle.clone())?;
            }
        }
    }
}
