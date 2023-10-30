use home;

pub fn appdata_dir() -> String {
    let mut home = home::home_dir().unwrap().to_str().unwrap().to_owned();
    home.push_str("/.concierge");
    home
}
