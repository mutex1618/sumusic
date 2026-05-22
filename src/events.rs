use crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use std::path::{Path, PathBuf};
use crate::{file::MusicItem, player::{self, PlayerCommand, start_player}};
use color_eyre::{Result, eyre::Ok};
use std::sync::mpsc;
use std::sync::mpsc::SendError;
pub struct App {
    pub music_list: Vec<MusicItem>,
    pub list_state: ListState,
    pub should_quit: bool,
    tx:mpsc::Sender<player::PlayerCommand>,
}
enum Mode{
    Normal,
    Insert,
    Processing,
}
impl App {
    pub fn new(path: &Path,tx:mpsc::Sender<PlayerCommand>) -> Self {
        let music_list = MusicItem::collect_files(path);
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            music_list,
            list_state,
            should_quit: false,
            tx,
        }
    }
    fn scroll_down(&mut self) {
        if !self.music_list.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => (i + 1).min(self.music_list.len() - 1),
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    fn scroll_up(&mut self) {
        if !self.music_list.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => i.saturating_sub(1),
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }
 pub fn handle_ui_events(&mut self, event: &Event) -> Result<()> {
    match event {
        Event::Key(key) => {
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
 pub fn map_key_event(
    &mut self,
    event: &Event,
) -> Option<Result<(), SendError<PlayerCommand>>> {
    let Event::Key(key) = event else { return None };

    match key.code {
        KeyCode::Enter => {
            let path = self.get_file()?;
            Some(self.tx.send(PlayerCommand::Play { path }))
        }
        KeyCode::Char('s') | KeyCode::Pause => Some(self.tx.send(PlayerCommand::Pause)),
        _ => None,
    }
}
pub fn handle_total_events(&mut self,event:Event){
    self.handle_ui_events(&event);
    self.map_key_event(&event);
}

fn get_file(&mut self) -> Option<PathBuf> {
    let entry = self
        .list_state
        .selected()
        .and_then(|i| self.music_list.get(i))?;

    if entry.is_audio {
        Some(entry.path.to_path_buf())
    } else {
        None
    }
}


}