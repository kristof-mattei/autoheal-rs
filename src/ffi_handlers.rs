use std::io::Error;
use std::ptr::null_mut;

use crate::wrap_and_report;
use color_eyre::eyre;
use libc::{SIG_IGN, SIGINT, SIGPIPE, SIGTERM, c_int, sigaction};
use tracing::{Level, event};

#[unsafe(no_mangle)]
pub extern "C" fn sig_handler(signal: i32) {
    event!(Level::INFO, raw_signal = signal, "Stopping the engine");
    // std::process::exit(128 + signal);
    // TODO proper shutdown
    #[expect(clippy::exit, reason = "Mirror existing application")]
    std::process::exit(0);
    // RUNNING.store(false, Ordering::SeqCst);
}

pub fn set_up_handler(signum: c_int, sig_handler_ptr: usize) -> Result<(), eyre::Report> {
    #[cfg(not(target_os = "macos"))]
    // SAFETY: all zeroes are valid for `sigset_t`
    let sa_mask = unsafe { std::mem::MaybeUninit::<libc::sigset_t>::zeroed().assume_init() };

    #[cfg(target_os = "macos")]
    let sa_mask = 0;

    let sa = sigaction {
        sa_sigaction: sig_handler_ptr,
        sa_flags: 0,
        sa_mask,
        #[cfg(not(target_os = "macos"))]
        sa_restorer: None,
    };

    // SAFETY: libc call
    if unsafe { sigaction(signum, &raw const sa, null_mut()) } == -1 {
        return Err(wrap_and_report!(
            Level::ERROR,
            Error::last_os_error(),
            "Failure to install signal handler"
        ));
    }

    Ok(())
}

pub(crate) fn set_up_handlers() -> Result<(), eyre::Report> {
    set_up_handler(SIGPIPE, SIG_IGN)?;

    #[expect(
        clippy::fn_to_numeric_cast_any,
        reason = "We actually need the function as a pointer, and this is well-defined"
    )]
    let sig_handler_ptr = sig_handler as usize;

    set_up_handler(SIGTERM, sig_handler_ptr)?;
    set_up_handler(SIGINT, sig_handler_ptr)?;

    Ok(())
}
