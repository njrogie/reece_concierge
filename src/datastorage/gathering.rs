use std::io::ErrorKind;

use chrono::{NaiveDate, Utc}; 
use serenity::model::prelude::Message;
use serenity::model::{id::ChannelId, id::GuildId};
use serenity::client::Context;
use substring::Substring;
use serde_json::Result;


enum ArgKind {
    ExcludeBefore(NaiveDate),
    ExcludeAfter(NaiveDate),
    ExcludeChannel(ChannelId)
}

pub struct SearchConstraints {
    pub before_date: NaiveDate,
    pub after_date: NaiveDate,
    pub allowed_channels: Vec<ChannelId> // might end up being strings eventually
}
 
impl SearchConstraints {
    pub fn new() -> SearchConstraints {
        SearchConstraints { 
            before_date: NaiveDate::default(),
            after_date: NaiveDate::default(),
            allowed_channels: Vec::<ChannelId>::new()
        }
    }

    pub async fn store_channels(&self, ctx: &Context, guild: &GuildId) -> core::result::Result<&str,&str> {
        let channel_list = guild.channels(&ctx.http).await;
        match channel_list {
            Ok(it) => {
                for (ch,guild_ch) in it {
                    match ch.name(&ctx.cache).await {
                        Some(name) => {
                            if name == "robot-zone" {
                                // Aggregate messages
                                let messages = ch.messages(ctx, |b|b).await.unwrap();
                                self.process_messages(messages);
                                return Ok("Done")
                            }
                        }, 
                        None => { }
                    }
                    
                }
            },
            Err(e) => { 
                return Err("Could not get channel list");
            }
        }
        return Ok("Test")
    }

    // attempts to store a set of args into search constraints.
    pub async fn parse_args(&mut self, args_str: String) -> core::result::Result<String,String>{
        // Split the string by space.
        let words = args_str.split(' ').collect::<Vec<&str>>();
        // Pair the arg-param pairs together.
        let result = words.iter().zip(words.iter().skip(1));
        let pairs = result.collect::<Vec<_>>();

        for pair in pairs {
            // check for argument type
            if String::from(pair.0.to_owned()).starts_with("--") {
                let arg = Self::parse_command(pair.0.to_owned(), pair.1.to_owned());
                
                if let Ok(arg_kind) = arg {
                    match arg_kind {
                        ArgKind::ExcludeAfter(cmd) => self.after_date = cmd ,
                        ArgKind::ExcludeBefore(cmd) =>  self.before_date = cmd,
                        ArgKind::ExcludeChannel(cmd) => self.allowed_channels.push(cmd)
                    }
                } else if let Err(e) = arg {
                    return Err(format!("One or more arguments could not be parsed: {} ({})", pair.0.to_owned(), e));
                }
            } 
        }

        if self.validate() {
            Ok(String::from("Arguments Parsed."))
        } else {
            Err(String::from("Not enough arguments added"));
        }
        
    }

    fn parse_command(arg: &str, param: &str) -> core::result::Result<ArgKind, std::io::Error> {
        let parse_date = |cmd: &str| -> NaiveDate {
            match NaiveDate::parse_from_str(cmd, "%Y-%m-%d") {
                Ok(it) => it,
                Err(_) => NaiveDate::default()
            }
        };

        match arg {
            "--exclude-before" => Ok(ArgKind::ExcludeBefore(parse_date(param))),
            "--exclude-after" => Ok(ArgKind::ExcludeAfter(parse_date(param))),
            "--exclude-channel" => Ok(ArgKind::ExcludeChannel(ChannelId::from({
                // To parse the channel, we have to get rid of the surrounding characters.
                let idx_start = param.find('#');
                let idx_end = param.find('>');

                if !idx_start.is_none() && !idx_end.is_none() {
                    let param_substr = param.substring(idx_start.unwrap() + 1, idx_end.unwrap());
                    match param_substr.parse::<u64>() {
                        Ok(num) => {println!("Parsed channel:{}", num); num},
                        Err(_) => { println!("Could not parse channel: {}", param); 0}
                    }
                } else {
                    println!("Could not parse channel: {}", param);
                    return Err(std::io::Error::new(ErrorKind::InvalidInput, "Bad ChannelId"));
                }               
            }))),
            _ => Err(std::io::Error::new(ErrorKind::InvalidInput,"Incorrect Argument Type"))
        }
    }

    fn process_messages(&self, msgs: Vec<Message>) -> Result<()> {
        // Firstly, output the messages to a test file so that we can process them in again.
        let j = serde_json::to_string(&msgs[0])?;
        println!("{}",j);
        
        Ok(())
    }

    fn validate(&self) -> bool {
        return self.before_date != NaiveDate::default() && self.after_date != NaiveDate::default();
    }

    fn date_range(&self) -> (NaiveDate, NaiveDate) {
        let is_valid_date = |date: &NaiveDate| -> bool {
            date != &NaiveDate::default()
        };

        let valid_before = is_valid_date(&self.before_date);
        let valid_after = is_valid_date(&self.after_date);

        if !valid_after {
            let now = Utc::now().naive_local().date();
            if !valid_before  {
                return (NaiveDate::default(), now);
            } else {
                return (self.before_date, now);
            }
        } else {
            if !valid_before {
                return (NaiveDate::default(), self.after_date);
            } else {
                return (self.before_date,self.after_date)
            }
        }
    }
    
    // return true if allowed_channels contains the target channel.
    pub fn is_channelid_allowed(&self, channel: ChannelId) -> bool {
        for (_, ch) in self.allowed_channels.iter().enumerate() {
            if ch.as_u64() == channel.as_u64() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use futures::executor; 
    
    macro_rules! block {
        ($($x:expr),* ) => {
            executor::block_on($($x)*)
        };
    }


    #[test]
    fn test_parseargs_date() {
        let mut sc = SearchConstraints::new();

        block!(sc.parse_args("--exclude-before 2022-09-30 --exclude-after 2023-09-30".to_string())).unwrap();

        assert_eq!(sc.before_date, NaiveDate::from_ymd_opt(2022,9,30).unwrap());
        assert_eq!(sc.after_date, NaiveDate::from_ymd_opt(2023,9,30).unwrap());
    }
}