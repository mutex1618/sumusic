use color_eyre::Result;
use crossterm::event;
mod render;
mod file;
mod events;
mod player;
use std::path::Path;

use crate::player::start_player;
fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    
    let path = Path::new("/home/sahar/Music");
    let tx = start_player();
    let mut app = events::App::new(path,tx);
    loop {
        terminal.draw(|f|render::draw(f,&mut app))?;
        let events=event::read()?;
        app.handle_ui_events(&events);
        app.map_key_event(&events);
        if app.should_quit {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
