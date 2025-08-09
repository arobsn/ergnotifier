use std::{error::Error, fs::File, path::Path};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub indexed_height: u64,
}

pub fn save(state: &AppState) -> Result<(), Box<dyn Error>> {
    let path = Path::new("state.json");
    save_to_disk(path, state)?;
    Ok(())
}

pub fn load() -> Result<AppState, Box<dyn Error>> {
    let state = load_from_disk(Path::new("state.json"))?;
    Ok(state)
}

fn save_to_disk<T: Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

fn load_from_disk<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn Error>> {
    let file = File::open(path)?;
    let data = serde_json::from_reader(file)?;
    Ok(data)
}
