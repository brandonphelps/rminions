#![allow(dead_code)]

use chrono::NaiveDateTime;
use postgres::Client as psqlClient;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoResponse {
    videos: Vec<Video>,

    /// token to pass in for next set of videos.
    next_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    url: String,
    title: String,
    video_id: String,
    posted_time: String,
    watched: Option<bool>,
}

impl Video {
    pub fn new(url: String, title: String, video_id: String, posted_time: String) -> Self {
        Self {
            url: url,
            title: title,
            video_id: video_id,
            posted_time: posted_time,
            watched: Some(false),
        }
    }

    pub fn get_video_id(&self) -> String {
        self.video_id.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    id: i32,
    title: String,
    channel_id: String,
    channel_type: String,
}

impl Channel {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_channel_id(&self) -> String {
        self.channel_id.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Network {
    title: String,
}

pub fn get_channel(conn: &mut psqlClient, channel_title: String) -> Option<Channel> {
    let s = "select * from channel where title=$1";
    for i in conn.query(s, &[&channel_title]).unwrap() {
        println!("{:#?}", i);

        let id: i32 = i.get(0);
        let title: &str = i.get(1);
        let _network_id: i32 = i.get(2);
        let channel_type: &str = i.get(3);
        let _hidden: bool = i.get(4);
        let channel_id: &str = i.get(5);

        return Some(Channel {
            id: id,
            title: title.into(),
            channel_id: channel_id.into(),
            channel_type: channel_type.into(),
        });
    }
    None
}

pub fn add_video(conn: &mut psqlClient, channel_id: i32, vid: Video) {
    let insert_s = "insert into video (channel_id, video_id, watched, posted_time, hidden, title, url) values($1, $2, $3, $4, $5, $6, $7)";
    let timezone = NaiveDateTime::parse_from_str(&vid.posted_time, "%Y-%m-%dT%H:%M:%SZ").unwrap();

    conn.execute(
        insert_s,
        &[
            &channel_id,
            &vid.video_id,
            &false,
            &timezone,
            &false,
            &vid.title,
            &vid.url,
        ],
    )
    .unwrap();
}

pub fn does_video_exist(conn: &mut psqlClient, video_id: String) -> bool {
    let where_s = "select video_id from video where video_id=$1";

    let mut res = false;

    for i in conn.query(where_s, &[&video_id]).unwrap() {
        let vid_id: &str = i.get(0);
        // likely redudant check.
        if vid_id == video_id {
            res = true;
        }
    }
    res
}

pub struct VideoFetcher {
    title: String,
    channel_id: String,
    token: Option<String>,
    current_videos: Vec<Video>,
    // indicates if we already proccessed first round of None
    visited_none: bool,
}

/// Must impl iterator and have it such that the order of the videos
/// returned is in chronological order according to the posted date.
impl VideoFetcher {
    pub fn new(title: String, channel_id: String) -> Self {
        Self {
            title: title,
            channel_id: channel_id,
            token: None,
            current_videos: Vec::new(),
            visited_none: false,
        }
    }
}

impl Iterator for VideoFetcher {
    type Item = Video;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_videos.len() == 0 {
            let client = Client::new();
            let url = match self.token {
                Some(ref token) => {
                    format!(
                        "http://192.168.0.4:5000/{}/{}/{}",
                        self.title, self.channel_id, token
                    )
                }
                None => {
                    if self.visited_none {
                        return None;
                    } else {
                        self.visited_none = true;
                        format!("http://192.168.0.4:5000/{}/{}", self.title, self.channel_id)
                    }
                }
            };
            // only need to fetch the first set of videos
            // and not all videos for sorting since
            // the youtube api will return with the videos
            // in sorted order already
            println!("Fetching more content: {}", url);
            let res = client.get(&url).send().unwrap();
            let mut j: VideoResponse = match res.json() {
                Ok(r) => r,
                Err(f) => {
                    panic!("{}: {}", f, url)
                }
            };

            if j.videos.len() == 0 {
            } else {
                self.token = j.next_token;
                // let temp = j.videos.iter().rev().collect();
                // todo: how to insert int reverse order
                self.current_videos.append(&mut j.videos);
                self.current_videos.reverse();
            }
        }

        if self.current_videos.len() > 0 {
            self.current_videos.pop()
        } else {
            println!("None");
            None
        }
    }
}

pub fn get_networks(conn: &mut postgres::Client) -> Vec<String> {
    let select = "select title from network";
    let mut rest = Vec::new();
    for i in conn.query(select, &[]).unwrap() {
        let title: &str = i.get(0);
        rest.push(title.into())
    }
    rest
}
