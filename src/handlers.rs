use std::io::Error;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use libc::c_int;
use libc::sigaction;
use libc::sigset_t;
use libc::SIGINT;
use libc::SIGPIPE;
use libc::SIGTERM;
use libc::SIG_IGN;
use tracing::event;
use tracing::Level;

use crate::wrap_and_report;

#[no_mangle]
pub extern "C" fn sig_handler(signal: i32) {
    event!(
        Level::INFO,
        message = "Stopping the engine",
        raw_signal = signal
    );
    // std::process::exit(128 + signal);
    std::process::exit(0);
    // RUNNING.store(false, Ordering::SeqCst);
}

fn set_up_handler(signum: c_int, handler: usize) -> Result<(), anyhow::Error> {
    let sa = sigaction {
        sa_sigaction: handler,
        sa_flags: 0,
        sa_mask: unsafe { MaybeUninit::<sigset_t>::zeroed().assume_init() },
        #[cfg(not(target_os = "macos"))]
        sa_restorer: None,
    };

    if unsafe { sigaction(signum, &sa, null_mut()) } == -1 {
        return Err(wrap_and_report!(
            Level::ERROR,
            Error::last_os_error(),
            "Failure to install signal handler"
        ));
    }

    Ok(())
}

pub(crate) fn set_up_handlers() -> Result<(), anyhow::Error> {
    set_up_handler(SIGPIPE, SIG_IGN)?;
    set_up_handler(SIGTERM, sig_handler as usize)?;
    set_up_handler(SIGINT, sig_handler as usize)?;

    Ok(())
}
