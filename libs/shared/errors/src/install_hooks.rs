use cfg_if::cfg_if;
use color_eyre::{config::HookBuilder, eyre::Result};
use tui::TuiRunner;

use crate::{install_color_eyre_panic_hook, install_eyre_hook};

pub fn install_hooks(tui: TuiRunner) -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();

    cfg_if! {
        if #[cfg(debug_assertions)] {
            crate::install_better_panic();
        } else {
            human_panic::setup_panic!();
        }
    }

    install_color_eyre_panic_hook(tui, panic_hook);
    install_eyre_hook(tui, eyre_hook)?;

    Ok(())
}
