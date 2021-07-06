#![allow(unused_imports)]

use std::collections::HashMap;

mod vidlid_db;

use postgres::{Client as psqlClient, NoTls};

use vidlid_db::{add_video, does_video_exist, get_channel, VideoFetcher};

// fn main_t() {
//     //let p = get_channel(&mut ps_client, "overthegun".into());
//     let c_name: String = "DF".into();
//     let c = get_channel(&mut ps_client, c_name.clone()).expect("Failed to get channel");

//     let fetcher = VideoFetcher::new(c_name.clone(), c.get_channel_id());
//     let mut already_added_count = 0;
//     let do_full = false;
//     for i in fetcher {
//         if does_video_exist(&mut ps_client, i.get_video_id()) {
//             already_added_count += 1;
//         } else {
//             add_video(&mut ps_client, c.get_id(), i);
//         }

//         if !do_full && already_added_count > 40 {
//             break;
//         }
//     }

// }

// #[get("/")]
// fn index() -> Html<&'static str> {
//     Html(r#"See <a href="tera">Tera</a> or <a href="hbs">Handlebars</a>."#)
// }

// #[get("/")]
// fn hello(world: String) -> String {
//     format!("hello: {}", world)
// }

// #[get("/")]
// fn tera_index() -> Redirect {
//     Redirect::to(uri!("/tera", tera_hello(name = "your name")))
// }

// #[get("/hello/<name>")]
// fn tera_hello(name: &str) -> Template {
//     Template::render("tera/index")
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, hello])
//         .mount("/tera", routes![tera_index, tera_hello])
// }
