use std::io::Error;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use color_eyre::eyre;
use libc::{SIG_IGN, SIGINT, SIGPIPE, SIGTERM, c_int, sigaction, sigset_t};
use tracing::{Level, event};

use crate::wrap_and_report;

#[unsafe(no_mangle)]
pub extern "C" fn sig_handler(signal: i32) {
    event!(Level::INFO, raw_signal = signal, "Stopping the engine");
    // std::process::exit(128 + signal);
    std::process::exit(0);
    // RUNNING.store(false, Ordering::SeqCst);
}

fn set_up_handler(signum: c_int, handler: usize) -> Result<(), eyre::Report> {
    let sa = sigaction {
        sa_sigaction: handler,
        sa_flags: 0,
        sa_mask: unsafe { MaybeUninit::<sigset_t>::zeroed().assume_init() },
        #[cfg(not(target_os = "macos"))]
        sa_restorer: None,
    };

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
        reason = "We actually need the function as a pointer"
    )]
    let sig_handler_ptr = sig_handler as usize;

    set_up_handler(SIGTERM, sig_handler_ptr)?;
    set_up_handler(SIGINT, sig_handler_ptr)?;

    Ok(())
}
