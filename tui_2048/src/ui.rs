use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let area = ratatui::prelude::Rect {
        x: f.size().width / 4,
        y: f.size().height / 4,
        width: f.size().width / 2,
        height: f.size().height / 2,
    };
    f.render_widget(
        Paragraph::new(format!("Press `Esc` or `Ctrl-C` to Quit.\nSearch: "))
            .block(
                Block::default()
                    .title("Launcher App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left),
        area,
    )
}
