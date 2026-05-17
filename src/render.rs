use std::path::Path;
use ratatui::style::{Style};
use ratatui::prelude::*;
use ratatui::widgets::{Block, List,ListState, ListDirection, ListItem,BorderType,Borders};
use crate::{file::MusicItem,events::App};
use ratatui::Frame;

pub fn draw(f:&mut Frame<'_>,list:&mut App){
    let music_bar=Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![Constraint::Percentage(40),Constraint::Percentage(60)])
    .split(f.area());
    let musics=List::new(list.music_list.iter().map(|m| m.name.to_string()).collect::<Vec<String>>())
    .block(Block::bordered())
    .style(Style::new().white().italic().fg(Color::Magenta))
    .highlight_style(Style::new().italic())
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true);
    let music_progress=Block::default()
    .border_style(Style::default().fg(Color::Magenta))
    .border_type(BorderType::Rounded)
    .borders(Borders::ALL);
    
    f.render_stateful_widget(musics,music_bar[0],&mut list.list_state);
    f.render_widget(music_progress, music_bar[1]);
}
