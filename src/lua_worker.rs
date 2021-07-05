use std::thread;
use std::time::Duration;

use std::sync::{mpsc, Arc, Mutex};

use rlua::Lua;

// todo: move this to vidlid_db
use postgres::{Client as psqlClient, NoTls};

use crate::vidlid_db::VideoFetcher;
use crate::vidlid_db::{get_channel, does_video_exist, add_video};

fn populate_db(channel_name: String) {
    //let p = get_channel(&mut ps_client, "overthegun".into());
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
}


pub enum LuaMessage {
    
}

pub enum LuaResponse {
}

pub struct LuaWorker {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl LuaWorker {
    fn initial_lua(lua: &Lua) {
        lua.context(|lua_ctx| { 
            let globals = lua_ctx.globals();
            let pop_db = lua_ctx.create_function(|_, chn_name: String| {
                println!("hello world: {}", chn_name);
                populate_db(chn_name);
                Ok(())
            }).unwrap();
            globals.set("populate_db", pop_db).unwrap();
        });
    }


    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<String>>>,
           sender: Arc<Mutex<mpsc::Sender<String>>>) -> Self {
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
                } else if message == "vidlid_update(overthegun)" {
                    populate_db("overthegun".into());
                    continue;
                } else if message == "vidlid_update(gura)" {
                    populate_db("gura".into());
                    continue;
                }


                lua.context(|lua_ctx| {
                    let lua_ret = lua_ctx.load(&message).eval::<rlua::MultiValue>();
                    
                    match lua_ret {
                        Ok(r) => {
                            
                            for j in r.iter() {
                                match *j {
                                    rlua::Value::Nil => {
                                        {
                                            println!("Got nil");
                                            sender.lock().unwrap().send("nil".into()).unwrap();
                                        }
                                    },
                                    rlua::Value::Boolean(t) => {
                                        {
                                            println!("Got bool");
                                            sender.lock().unwrap().send(format!("{}", t)).unwrap();
                                        }
                                    },
                                    rlua::Value::Integer(t) => {
                                        {
                                            println!("Got integer");
                                            sender.lock().unwrap().send(format!("{}", t)).unwrap();
                                        }
                                    },
                                    _ => {
                                        {
                                            println!("Got unknown");
                                            sender.lock().unwrap().send("unknown string".into()).unwrap();
                                        }
                                    }
                                }
                            }
                        },
                        Err(r) => {
                            println!("got an error");
                        },
                    };
                });
                thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            thread: Some(p)
        }
    }
}
