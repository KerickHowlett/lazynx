use color_eyre::{config::EyreHook, eyre};
use tui::TuiRunner;

pub fn install_eyre_hook(tui: TuiRunner, eyre_hook: EyreHook) -> color_eyre::Result<()> {
    let tui = tui.clone();
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        tui.restore().unwrap();
        eyre_hook(error)
    }))?;
    Ok(())
}
