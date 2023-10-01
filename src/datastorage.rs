
/*
    For now, this is going to be a singular, non-scalable application.
    So the interfaces are going to write to files. 

    Eventually, public functions shall send data to a database and this will be a singular app
*/
use std::{fs, path::PathBuf};
use std::io::{Error, ErrorKind, Write};
use home;

mod gathering; 

pub fn init(){
    let appdata = appdata_dir().to_string();
    mkdir_if_not_exist(appdata);
}

pub fn save_channelid(guild: u64, channel: String) {
    mkdir_if_not_exist(appdata_dir().to_owned() + "/" + &guild.to_string());

    let mut res = fs::OpenOptions::new();
    let createfile = res.create(true).write(true);
    let path = channelid_filepath(guild.to_string());
    println!("{}", path.display());
    let openedfile = createfile.open(path);

    match openedfile {
        Err(err) => { println!("could not write to file! {}",err); },
        Ok(mut it) => { 
            let _ = it.write_all(channel.as_bytes()); 
            let _ = it.flush(); 
        }
    }

}

// Get the posting channel ID associated with the guild
// (The bot should only be allowed to post in one channel per guild)
pub fn get_channelid(guild: String) -> Result<String, std::io::Error> {
    let read_file_out = 
        fs::read_to_string(channelid_filepath(guild.to_string()));

    return match read_file_out {
        Ok(good) => Ok(good),
        Err(_) => Err(Error::new(
            ErrorKind::NotFound,
            "Could not read channel id"))
    }
}

fn create_dir(path: String) {
    println!("path:{}", path);
    match fs::create_dir(path) {
        Ok(_) => println!("Successfully created appdata dir."),
        Err(_) => println!("Could not create appdata dir."),
    }
}

fn mkdir_if_not_exist(dir_name: String) {
    
    match fs::metadata(&dir_name) {
        Ok(_) => println!("{} dir exists", dir_name),
        Err(_) => {
            create_dir(dir_name);
            return
        }
    }
}

fn channelid_filepath(guild: String) -> PathBuf {
    let appdata = appdata_dir();
    
    let mut appdata = PathBuf::from(appdata);
    appdata.push(guild + "/channel_id");
    appdata
}

fn appdata_dir() -> String {
    let mut home = home::home_dir().unwrap().to_str().unwrap().to_owned();
    home.push_str("/.concierge");
    home
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_savechannelid() {
        save_channelid(100, "test".to_string());
        assert!({
            match fs::metadata("/home/njrogie/.concierge/100/channel_id") {
                Ok(_) => true,
                Err(_) => {
                    false
                }
            }
        })
    }

    #[test]
    fn test_channelid() {
        assert_eq!(channelid_filepath("guild".to_string()), PathBuf::from("/home/njrogie/.concierge/guild/channel_id"));
    }

    #[test]
    fn test_appdata() {
        assert_eq!(appdata_dir(), "/home/njrogie/.concierge");
    }

    #[test]
    fn test_dateparsing() {

    }
}