use chrono::NaiveDate; 
use serenity::model::id::ChannelId;
pub struct SearchConstraints {
    pub before_date: NaiveDate,
    pub after_date: NaiveDate,
    pub allowed_channels: Vec<ChannelId> // might end up being strings eventually
}

impl SearchConstraints {
    // return true if allowed_channels contains the target channel.
    pub fn is_channelid_allowed(&self, channel: ChannelId) -> bool {
        for (sz, ch) in self.allowed_channels.iter().enumerate() {
            if(ch.as_u64() == channel.as_u64()) {
                return true;
            }
        }
        false
    }

    // attempts to store a set of args into search constraints.
    pub fn parse_args(&mut self, args_str: String) {

    }
/* 
    fn parse_before_date(&self, date_str: String) -> NaiveDate {
        
    }
    */
}

#[cfg(test)]
mod tests{
    use serenity::model::prelude::Channel;

    use super::*;
    fn test_parseargs() {
        let mut sc = SearchConstraints { 
            before_date: NaiveDate::from_ymd_opt(0,0,0).unwrap(),
            after_date:NaiveDate::from_ymd_opt(0,0,0).unwrap(),
             allowed_channels: Vec::<ChannelId>::new()};
        sc.parse_args("--exclude-before 09/30/2022\n--exclude-after 09/30/2023".to_string());
    }
}