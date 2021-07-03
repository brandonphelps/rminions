

mod vidlid_db;

use postgres::{Client as psqlClient, NoTls};

use vidlid_db::{VideoFetcher,  get_channel, does_video_exist, add_video};

fn main() {


    //let p = get_channel(&mut ps_client, "overthegun".into());
    let c_name: String = "DF".into();
    let c = get_channel(&mut ps_client, c_name.clone()).expect("Failed to get channel");

    let fetcher = VideoFetcher::new(c_name.clone(), c.get_channel_id());
    let mut already_added_count = 0;
    let do_full = false;
    for i in fetcher {
        // println!("{:#?}", i);
        if does_video_exist(&mut ps_client, i.get_video_id()) {
           // println!("Video already exists");
            already_added_count += 1;
        } else {
            //println!("Should add video");
            add_video(&mut ps_client, c.get_id(), i);
        }
        
        if !do_full && already_added_count > 40 { 
            break;
        }
    }

}
