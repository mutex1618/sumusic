use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug,Clone)]
pub struct MusicItem {
    pub is_audio: bool,
    pub is_dir: bool,
    pub name: String,  
    pub path: PathBuf,
}
impl MusicItem{
pub fn collect_files(dir_path: &Path) -> Vec<MusicItem> {
    let mut items = Vec::new();
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        // Get the file/directory name
        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string();
        let is_audio = path.extension()
            .map(|ext| {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(ext.as_str(), "mp3" | "wav" | "flac" | "m4a" | "ogg")
            })
            .unwrap_or(false);
        let musicitem=MusicItem{
            is_audio,
            is_dir:metadata.is_dir(),
            name,
            path
        };
        items.push(musicitem);
    }
    items
}
}