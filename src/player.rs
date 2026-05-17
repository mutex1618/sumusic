use rodio::{Decoder, DeviceSinkBuilder,Player, Source};
use std::fs::File;
use std::io::{BufReader, sink};
use std::path::{Path, PathBuf};
use std::sync:: mpsc;
use std::time::Duration;
use std::thread;

pub enum PlayerState{
    Idle,
    Playing,
    Stopped,
}
pub enum PlayerCommand {
    Play
    {path:PathBuf},
    Pause,
    Resume,
    Stop,
    Seek(Duration),
    Shutdown,
}
pub fn start_player()->mpsc::Sender<PlayerCommand>{
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        player(rx);
    });
    tx
}
pub fn player(rx:mpsc::Receiver<PlayerCommand>){
    thread::spawn( move||{
        let sink_handle=rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player:Player=rodio::Player::connect_new(&sink_handle.mixer());
        let mut state=PlayerState::Idle;
        while let Ok(cmd)=rx.recv(){
        match cmd{
        PlayerCommand::Play{path}=>{
            let file = BufReader::new(File::open(path).unwrap());
            let source=Decoder::try_from(file).expect("couldn't convert file to signal.");
            
            match state{
                PlayerState::Playing=>{
                player.stop();
                player.append(source);
                player.play();
            }
            PlayerState::Stopped | PlayerState::Idle=>{
                player.append(source);
                player.play();
            }
        }
            state=PlayerState::Playing;
        },
        PlayerCommand::Pause=>{
            player.pause();
            state=PlayerState::Stopped;
        },
        PlayerCommand::Resume=>{
            player.play();
            state=PlayerState::Playing;
        }
        _=>{}
    }
        }
    } 
    );
}