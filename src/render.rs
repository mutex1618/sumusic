use std::path::Path;
use rand::{Rng, RngExt};
use ratatui::symbols;
use ratatui::style::{Style};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use crate::{file::MusicItem,events::App};
use ratatui::Frame;
pub fn draw(f:&mut Frame<'_>,list:&mut App){
    let music_bar=Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![Constraint::Percentage(40),Constraint::Percentage(60)])
    .split(f.area());
    let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
    let cv = block.inner(music_bar[1]);
    let text_state=Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)])
    .split(cv);
    let musics=List::new(list.music_list.iter().map(|m| m.name.to_string()).collect::<Vec<String>>())
    .block(Block::bordered())
    .style(Style::new().white().italic().fg(Color::Magenta))
    .highlight_style(Style::new().italic())
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true);
    let show_state=Paragraph::new("Welcome to Sumusic!✨️")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Cyan));
    let sin_wave=Canvas::default()
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 0.0])
    .marker(symbols::Marker::Quadrant)
    .paint(|ctx| {
        for i in (-180..180).step_by(10){
        let h=40.0*(i as f64/15.0).sin();
        ctx.draw(&Rectangle {
            x:i as f64,
            y:-90.0,
            width:3.0,
            height:h,
            color:Color::LightRed,
        });
        ctx.layer();
    }
        });
    f.render_stateful_widget(musics,music_bar[0],&mut list.list_state);
    f.render_widget(show_state, text_state[0]);
    f.render_widget(sin_wave, text_state[1]);
}
