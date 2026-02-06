use crate::vault::Vault;
use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    View,
    Add,
    ConfirmDelete,
}

pub struct App {
    pub vault: Vault,
    pub list_state: ListState,
    pub should_quit: bool,
    pub mode: Mode,

    // temp buffers
    pub input_service: String,
    pub input_username: String,
    pub input_password: String,
}

impl App {
    pub fn new(vault: Vault) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            vault,
            list_state,
            should_quit: false,
            mode: Mode::Normal,
            input_service: String::new(),
            input_username: String::new(),
            input_password: String::new(),
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn selected_service(&self) -> Option<&str> {
        let i = self.selected_index()?;
        self.vault.list_services().get(i).copied()
    }

    pub fn next(&mut self) {
        let len = self.vault.list_services().len();
        if len == 0 { return; }
        let i = self.selected_index().unwrap_or(0);
        self.list_state.select(Some((i + 1) % len));
    }

    pub fn previous(&mut self) {
        let len = self.vault.list_services().len();
        if len == 0 { return; }
        let i = self.selected_index().unwrap_or(0);
        self.list_state.select(Some(if i == 0 { len - 1 } else { i - 1 }));
    }

    pub fn clear_inputs(&mut self) {
        self.input_service.clear();
        self.input_username.clear();
        self.input_password.clear();
    }
}
