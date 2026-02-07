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

    let app = App::new(vault);

    let app = ui::start(app)?;

    storage::save_vault(path, &app.vault, &master, &salt)?;

    Ok(())
}
