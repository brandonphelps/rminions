#![allow(dead_code)]
#![allow(unused_imports)]

use std::thread;
use std::time::Duration;

use std::sync::{mpsc, Arc, Mutex};

use rlua::Lua;

// todo: move this to vidlid_db
use postgres::{Client as psqlClient, NoTls};


use crate::vidlid_db::VideoFetcher;
use crate::vidlid_db::{get_channel, get_channels as o_get_channels, does_video_exist, add_video};

fn get_channels() -> Vec<String> {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    o_get_channels(&mut ps_client)
}

fn populate_db(channel_name: String) -> Vec<String> {
    let mut ps_client = psqlClient::connect("host=192.168.0.4 user=postgres password=secretpassword port=5432", NoTls).unwrap();

    let c_name: String = channel_name;
    let c = get_channel(&mut ps_client, c_name.clone()).expect("Failed to get channel");

    let fetcher = VideoFetcher::new(c_name.clone(), c.get_channel_id());
    let mut already_added_count = 0;
    let do_full = false;
    for i in fetcher {
        if does_video_exist(&mut ps_client, i.get_video_id()) {
            println!("Already have video: {}", i.get_title());
            already_added_count += 1;
        } else {
            println!("Adding vid: {}", i.get_title());
            add_video(&mut ps_client, c.get_id(), i);
        }

        if !do_full && already_added_count > 40 {
            break;
        }
    }

    Vec::new()

// vec!["hello".into(), "world".into(), "i like cheeze".into()]

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

impl LuaWorker {
    fn initial_lua(lua: &Lua) {
        lua.context(|lua_ctx| { 
            let globals = lua_ctx.globals();
            let pop_db = lua_ctx.create_function(|_, chn_name: String| {
                let res = populate_db(chn_name);
                Ok(res)
            }).unwrap();
            let channel_list = lua_ctx.create_function(|_, ()| {
                Ok(get_channels())
            }).unwrap();
            globals.set("populate_db", pop_db).unwrap();
            globals.set("list_channels", channel_list).unwrap();
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
                                        LuaWorker::send_message(&sender, Some("nil".into()));
                                    },
                                    rlua::Value::Boolean(t) => {
                                        LuaWorker::send_message(&sender, Some(format!("{}", t)));
                                    },
                                    rlua::Value::Integer(t) => {
                                        LuaWorker::send_message(&sender, Some(format!("{}", t)));
                                    },
                                    rlua::Value::Table(t) => {
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
