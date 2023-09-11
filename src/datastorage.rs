use std::{fs, path::PathBuf};

const appdata_dir: &str = "~/.concierge";
const channel_id: &str = ""
pub fn init(){
    let appdata = appdata_dir.to_string();
    match fs::metadata(&appdata) {
        Ok(_) => println!("AppData dir exists"),
        Err(_) => {
            create_dir(appdata);
            return
        }
    }
}

pub fn save_channelid(id: String) {
    fs::write(channelid_filepath(), id);
}

pub fn get_channelid() -> String {
    fs::read(file)
}

fn create_dir(path: String) {
    match fs::create_dir(path) {
        Ok(_) => println!("Successfully created appdata dir."),
        Err(_) => println!("Could not create appdata dir."),
    }
}

fn channelid_filepath() -> PathBuf {
    PathBuf::from(appdata_dir).join("channel_id")
}