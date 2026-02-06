pub mod app;
pub mod ui;
pub mod events;

use std::path::Path;
use anyhow::Result;

use crate::storage;
use crate::tui::app::App;

pub fn run(path: &Path) -> Result<()> {
    let master = rpassword::prompt_password("Master password: ")?;
    let (vault, salt) = storage::load_vault(path, &master)?;

    let mut app = App::new(vault);

    // run TUI
    ui::start(app)?;

    // save vault on exit
    storage::save_vault(path, &app.vault, &master, &salt)?;

    Ok(())
}
