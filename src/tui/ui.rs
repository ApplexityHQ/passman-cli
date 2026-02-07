use crate::tui::app::{App, Mode};
use crate::tui::events::handle_events;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Style, Modifier},
    layout::{Layout, Constraint, Direction},
};

use std::io::stdout;

pub fn start(mut app: App) -> Result<App> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(size);

            // LIST
            let services = app.vault.list_services();

            if services.is_empty() {
                let empty = Paragraph::new(
                    "🔐 Vault is empty\n\nPress 'a' to add a new entry\nPress 'q' to quit"
                )
                .block(Block::default().title("passman 🔐").borders(Borders::ALL))
                .alignment(ratatui::layout::Alignment::Center);

                f.render_widget(empty, chunks[0]);
            } else {
                let items: Vec<ListItem> =
                    services.iter().map(|s| ListItem::new(*s)).collect();

                let list = List::new(items)
                    .block(Block::default().title("passman 🔐").borders(Borders::ALL))
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .highlight_symbol("> ");

                f.render_stateful_widget(list, chunks[0], &mut app.list_state);
            }

            // STATUS BAR
            let status = match app.mode {
                Mode::Normal =>
                    "NORMAL | ↑↓ navigate | Enter view | a add | d delete | q quit",
                Mode::View =>
                    "VIEW MODE | Esc to close",
                Mode::Add =>
                    "ADD MODE | Type password | Enter save | Esc cancel",
                Mode::ConfirmDelete =>
                    "CONFIRM DELETE | y confirm | n cancel",
            };


            f.render_widget(Paragraph::new(status), chunks[1]);

            // POPUPS
            if app.mode == Mode::View {
                if let Some(service) = app.selected_service() {
                    if let Some(entry) = app.vault.get_entry(service) {
                        let popup = Paragraph::new(format!(
                            "Service: {}\nUser: {}\nPassword: {}",
                            entry.service, entry.username, entry.password
                        ))
                        .block(Block::default().title("Entry").borders(Borders::ALL));

                        f.render_widget(popup, centered_rect(60, 40, size));
                    }
                }
            }

        if app.mode == Mode::Add {
            let form = Paragraph::new("🔥 ADD MODE ACTIVE 🔥\n\nType something.\nTab = next field\nEsc = cancel")
                .block(
                    Block::default()
                        .title("ADD ENTRY")
                        .borders(Borders::ALL)
                );

            f.render_widget(form, centered_rect(60, 40, size));
        }

        })?;

        handle_events(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(app)
}

// helper
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::prelude::Rect)
    -> ratatui::prelude::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
