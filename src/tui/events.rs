use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

use crate::tui::app::{App, Mode, AddField};
use crate::models::PasswordEntry;

pub fn handle_events(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key) => match app.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('q') => app.should_quit = true,
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => app.mode = Mode::View,
                KeyCode::Char('a') => app.mode = Mode::Add,
                KeyCode::Char('d') => app.mode = Mode::ConfirmDelete,
                _ => {}
            },

            Mode::View => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => app.mode = Mode::Normal,
                _ => {}
            },

            Mode::ConfirmDelete => match key.code {
                KeyCode::Char('y') => {
                    if let Some(service) = app.selected_service().map(|s| s.to_string()) {
                        let _ = app.vault.delete_entry(&service);
                    }
                    app.mode = Mode::Normal;
                }
                KeyCode::Char('n') | KeyCode::Esc => app.mode = Mode::Normal,
                _ => {}
            },

            Mode::Add => match key.code {
                KeyCode::Esc => {
                    app.clear_inputs();
                    app.mode = Mode::Normal;
                }

                KeyCode::Tab => {
                    app.add_field = match app.add_field {
                        AddField::Service => AddField::Username,
                        AddField::Username => AddField::Password,
                        AddField::Password => AddField::Service,
                    };
                }

                KeyCode::Backspace => match app.add_field {
                    AddField::Service => { app.input_service.pop(); }
                    AddField::Username => { app.input_username.pop(); }
                    AddField::Password => { app.input_password.pop(); }
                }

                KeyCode::Enter => {
                    if !app.input_service.is_empty() {
                        let _ = app.vault.add_entry(PasswordEntry {
                            service: app.input_service.clone(),
                            username: app.input_username.clone(),
                            password: app.input_password.clone(),
                        });
                    }
                    app.clear_inputs();
                    app.mode = Mode::Normal;
                }

                KeyCode::Char(c) if c.is_ascii_graphic() => match app.add_field {
                    AddField::Service => app.input_service.push(c),
                    AddField::Username => app.input_username.push(c),
                    AddField::Password => app.input_password.push(c),
                }

                _ => {}
            },
        },
        _ => {}
    }

    Ok(())
}





