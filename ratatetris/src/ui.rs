#![allow(unused_imports)]
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
};
use std::{cmp, collections::HashMap};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(GRID_Y as u16 * 3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());
    let title = Paragraph::new("Game").block(title_block);
    frame.render_widget(title, vertical[0]);

    render_grid(frame, app, vertical[1]);
}

// won't be able to make it with nice squares; just ignore that squares are rectangles now
const GRID_Y: usize = 10;
const GRID_X: usize = 15;

fn render_grid(frame: &mut Frame, app: &App, area: Rect) {
    let box_size = cmp::min(area.height / GRID_Y as u16, area.width / GRID_X as u16);
    let rows = (0..GRID_Y).map(|y| {
        let row = (0..GRID_X)
            .map(|x| {
                let mut cell = Cell::from(Text::from(format!("({},{})", x, y)));
                // if y == 1 && x == 3 {
                cell = cell.on_red();
                // }
                cell
            })
            .collect::<Row>()
            .height(box_size * 2 / 3);
        row
    });
    let constraints = (0..GRID_X).map(|_| Constraint::Length(box_size));
    let table = Table::new(rows, constraints).block(
        Block::default()
            .borders(Borders::all())
            .style(Style::default())
            .title("Grid"),
    );
    frame.render_widget(table, area);
}
