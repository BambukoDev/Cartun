use ratatui::{Frame, text::Text, style::{Color, Stylize}, widgets::{Block, Borders}, layout::Constraint, palette::Hsl};
use crate::shared;
use chrono::prelude::*;

pub fn bg(frame: &mut Frame, time: DateTime<Local>) {
    let b = Block::default().bg(Color::from_hsl(Hsl::new(time.timestamp() as f32/86400.0, 1.0, 0.5))).borders(Borders::NONE);

    frame.render_widget(b, frame.area());
}

pub fn clock_1(frame: &mut Frame, time: DateTime<Local>) {
    let text = Text::raw(format!("{}", time.format("%H:%M:%S"))).fg(Color::from_hsl(Hsl::new((time.timestamp()+180) as f32/86400.0, 1.0, 0.1)));
    let area = shared::center(frame.area(), Constraint::Length(text.width() as u16), Constraint::Length(1));

    frame.render_widget(text, area);
}
