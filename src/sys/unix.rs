//! This module contains all `unix` specific terminal related logic.

use std::collections::HashMap;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Mutex;
use std::{io, mem};

pub use libc::termios as Termios;
use libc::{cfmakeraw, tcgetattr, tcsetattr, TCSANOW};

use lazy_static::lazy_static;

use crate::{ErrorKind, Result};

lazy_static! {
    // Some(Termios) -> we're in the raw mode and this is the previous mode
    // None -> we're not in the raw mode
    static ref TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<HashMap<RawFd, Termios>> = Mutex::new(HashMap::new());
}

pub fn is_raw_mode_enabled(file: &impl AsRawFd) -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE
        .lock()
        .unwrap()
        .contains_key(&file.as_raw_fd())
}

fn wrap_with_result(t: i32) -> Result<()> {
    if t == -1 {
        Err(ErrorKind::IoError(io::Error::last_os_error()))
    } else {
        Ok(())
    }
}

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

pub fn get_terminal_attr(file: &impl AsRawFd) -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        wrap_with_result(tcgetattr(file.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(file: &mut impl AsRawFd, termios: &Termios) -> Result<()> {
    wrap_with_result(unsafe { tcsetattr(file.as_raw_fd(), TCSANOW, termios) })
}

pub fn enable_raw_mode(file: &mut impl AsRawFd) -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if original_mode.contains_key(&file.as_raw_fd()) {
        return Ok(());
    }

    let mut ios = get_terminal_attr(file)?;
    let original_mode_ios = ios;

    raw_terminal_attr(&mut ios);
    set_terminal_attr(file, &ios)?;

    // Keep it last - set the original mode only if we were able to switch to the raw mode
    original_mode.insert(file.as_raw_fd(), original_mode_ios);

    Ok(())
}

pub fn disable_raw_mode(file: &mut impl AsRawFd) -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if let Some(original_mode_ios) = original_mode.get(&file.as_raw_fd()) {
        set_terminal_attr(file, original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        original_mode.remove(&file.as_raw_fd());
    }

    Ok(())
}
