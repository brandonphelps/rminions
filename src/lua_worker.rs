#![allow(dead_code)]
#![allow(unused_imports)]

use rlua::ToLua;
use std::thread;
use std::time::Duration;

use std::sync::{mpsc, Arc, Mutex};

use rlua::Lua;

use std::path::PathBuf;

// todo: move this to vidlid_db
use postgres::{Client as psqlClient, NoTls};


use crate::vidlid_db::{VideoFetcher, Video};
use crate::vidlid_db::{get_channel, get_videos,
                       get_channels as o_get_channels,
                       set_watched,
                       does_video_exist, add_video};

impl<'lua> ToLua<'lua> for Video {
    fn to_lua(self, con: rlua::Context<'lua>) -> std::result::Result<rlua::Value<'lua>, rlua::Error> {
        let mut table: rlua::Table = con.create_table().unwrap();
        table.set("title", self.get_title());

        Ok(rlua::Value::Table(table))
    }
}

fn mark_watched(video_title: String, watched: bool) {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    set_watched(&mut ps_client, video_title, watched);
}

fn get_channels() -> Vec<String> {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    o_get_channels(&mut ps_client)
}

fn list_videos_by_ch(channel_name: String) -> Vec<Video> {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    match get_videos(&mut ps_client, channel_name) {
        Some(ref video_list) => {
            let mut res = Vec::new();
            for i in video_list.iter() {
                res.push(i.clone());
            }
            res
        },
        None => { Vec::new() },
    }
}

fn populate_db(channel_name: String) -> Vec<String> {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    let c_name: String = channel_name;
    let c = get_channel(&mut ps_client, c_name.clone()).expect("Failed to get channel");

    let fetcher = VideoFetcher::new(c_name.clone(), c.1.get_channel_id());
    let mut already_added_count = 0;
    let do_full = false;
    let mut res = Vec::new();
    for i in fetcher {
        if does_video_exist(&mut ps_client, i.get_video_id()) {
            println!("Already have video: {}", i.get_title());
            already_added_count += 1;
            res.push(i.get_title());
        } else {
            res.push(format!("Adding: {}", i.get_title()));
            add_video(&mut ps_client, c.0, i);
        }

        if !do_full && already_added_count > 40 {
            break;
        }
    }
    res
}

pub enum LuaMessage {
    
}

pub enum LuaResponse {
    Done,
    NotDone,
    Result(String),
}

pub struct LuaWorker {
    pub thread: Option<thread::JoinHandle<()>>,
}

fn load_lua_file<P>(lua: &Lua, c: P)
where
    P: std::convert::AsRef<std::path::Path>,
{
    let file_contents = std::fs::read_to_string(c).expect("Failed to load lua file");
    lua.context(|lua_ctx| {
        lua_ctx.load(&file_contents).exec().expect("Failed to load file");
    });
}

impl LuaWorker {
    fn initial_lua(lua: &Lua) {
        lua.context(|lua_ctx| { 

            // move this handling into the lua_worker init 
            let mut lua_src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            lua_src_dir.push("lua");
            let lua_main = lua_src_dir.join("main.lua");
            
            let file_contents = std::fs::read_to_string(lua_main).expect("Failed to read from lua string");
            lua.context(|lua_ctx| {
                lua_ctx.load(&file_contents).exec().unwrap();
            });

            let globals = lua_ctx.globals();
            let pop_db = lua_ctx.create_function(|_, chn_name: String| {
                let res = populate_db(chn_name);
                Ok(res)
            }).unwrap();

            let channel_list = lua_ctx.create_function(|_, ()| {
                Ok(get_channels())
            }).unwrap();

            let r_print = lua_ctx.create_function(|_, val: String| {
                println!("Rust Print: {}", val);
                Ok(vec![val
                ])
            }).unwrap();
                
            let vide_list = lua_ctx.create_function(|_, chn_name: String| {
                Ok(list_videos_by_ch(chn_name))
            }).unwrap();
            let set_watcch = lua_ctx.create_function(|_, (vid_name, valu): (String, bool)| {
                Ok(mark_watched(vid_name, valu))
            }).unwrap();

            globals.set("populate_db", pop_db).unwrap();
            globals.set("list_channels", channel_list).unwrap();
            globals.set("list_videos", vide_list).unwrap();
            globals.set("r_print", r_print).unwrap();
            globals.set("set_watched", set_watcch).unwrap();
        });
    }

    fn send_message(sender: &Arc<Mutex<mpsc::Sender<LuaResponse>>>, res: Option<String>) {
        if let Some(t) = res {
            sender.lock().unwrap().send(LuaResponse::Result(t)).unwrap();
        } else {
            sender.lock().unwrap().send(LuaResponse::Done).unwrap();
        }
    }

    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<String>>>,
               sender: Arc<Mutex<mpsc::Sender<LuaResponse>>>) -> Self {
        let p = thread::spawn(move || {
            let lua = Lua::new();

            // load inital lua environment
            LuaWorker::initial_lua(&lua);

            loop {
                println!("Waiting for lua message");
                let message = receiver.lock().unwrap().recv().unwrap();
                println!("Got lua message: {}", message);
                if message == "l_exit" {
                    break;
                }

                lua.context(|lua_ctx| {
                    let lua_ret = lua_ctx.load(&message).eval::<rlua::MultiValue>();
                    
                    match lua_ret {
                        Ok(r) => {
                            for j in r.iter() {
                                match j {
                                    rlua::Value::Nil => {
                                        println!("Got nil");
                                        LuaWorker::send_message(&sender, Some("nil".into()));
                                    },
                                    rlua::Value::Boolean(t) => {
                                        println!("Got a bool");
                                        LuaWorker::send_message(&sender, Some(format!("{}", t)));
                                    },
                                    rlua::Value::Integer(t) => {
                                        println!("Got a int");
                                        LuaWorker::send_message(&sender, Some(format!("{}", t)));
                                    },
                                    rlua::Value::String(t) => {
                                        let val_r = format!("{}", t.to_str().unwrap());
                                        println!("Got a value back: {}", val_r);
                                        LuaWorker::send_message(&sender, Some(val_r));
                                    },
                                    rlua::Value::Table(t) => {
                                        println!("Got a table");
                                        let len: i64 = t.len().unwrap();
                                        for i in 1..len {
                                            match t.get::<i64, String>(i) {
                                                Ok(r) => {
                                                    LuaWorker::send_message(&sender,
                                                                            Some(r.into()));
                                                },
                                                Err(r) => { println!("Err: {:#?}", r) },
                                            }
                                        }
                                    },
                                    _ => {
                                        println!("Got unknown");
                                        LuaWorker::send_message(&sender, Some("unknown string".into()));
                                    }
                                }
                            }
                        },
                        Err(_r) => {
                            println!("got an error");
                        },
                    };
                });
                thread::sleep(Duration::from_secs(1));
                // send the Done message.
                LuaWorker::send_message(&sender, None);
            }
        });

        Self {
            thread: Some(p)
        }
    }
}
