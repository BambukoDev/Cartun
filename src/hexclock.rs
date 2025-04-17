use ratatui::{Frame, text::Text, style::{Color, Stylize}, widgets::{Block, Borders}, layout::Constraint, palette::Hsl};
use crate::shared;
use chrono::prelude::*;

pub fn bg(frame: &mut Frame, time: DateTime<Local>) {
    let b = Block::default().bg(Color::from_hsl(Hsl::new(time.timestamp() as f32/86400.0, 1.0, 0.5))).borders(Borders::NONE);

    frame.render_widget(b, frame.area());
}

pub fn clock_normal(frame: &mut Frame, time: DateTime<Local>) {
    let text = Text::raw(format!("{}", time.format("%H:%M:%S"))).fg(Color::from_hsl(Hsl::new((time.timestamp()+180) as f32/86400.0, 1.0, 0.1)));
    let area = shared::center(frame.area(), Constraint::Length(text.width() as u16), Constraint::Length(1));

    frame.render_widget(text, area);
}

pub fn clock_big(frame: &mut Frame, time: DateTime<Local>) {
    let time = time.format("%H:%M:%S").to_string();
    let mut out = String::new();

    for row in &shared::CLOCK_DIGITS {
        for c in time.chars() {
            let col = match c {
                '0'..='9' => c as usize - '0' as usize,
                _ => 10
            };
            out += row[col];
        }
        out += "\n";
    }

    let text = Text::raw(out);
    let area = shared::center(frame.area(), Constraint::Length(text.width() as u16), Constraint::Length(text.height() as u16));

    frame.render_widget(text, area);
}