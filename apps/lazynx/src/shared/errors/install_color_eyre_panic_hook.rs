use std::panic;

use color_eyre::config::PanicHook;
use tracing::error;

use crate::shared::tui::TuiRunner;

pub fn install_color_eyre_panic_hook(tui: TuiRunner, panic_hook: PanicHook) {
    // convert from a `color_eyre::config::PanicHook`` to a `Box<dyn
    // Fn(&PanicInfo<'_>`
    let tui = tui.clone();
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        if let Err(err) = tui.restore() {
            error!("Unable to restore terminal: {err:?}");
        }

        // not sure about this
        // let msg = format!("{}", panic_hook.panic_report(panic_info));
        // error!("Error: {}", strip_ansi_escapes::strip_str(msg));
        panic_hook(panic_info);
    }));
}
