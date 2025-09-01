use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new("App").block(title_block);

    frame.render_widget(title, chunks[0]);

    let content_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("Content");

    let content = Paragraph::new(Text::styled(
        app.string(),
        Style::default().fg(Color::White),
    ))
    .block(content_block);

    frame.render_widget(content, chunks[1]);
}
