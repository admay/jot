use std::env;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;

use std::error::Error;
use tinytemplate::TinyTemplate;

static TEMPLATE: &'static str = "{header}\n\n{timestamp}{message}\n{footer}";

#[derive(Serialize)]
struct Context {
    message: String,
    header: String,
    footer: String,
    timestamp: String,
}

pub fn handle(msg: String) -> Result<(), Box<dyn Error>> {
    let today: &str = &Local::today().format("%Y-%b-%d").to_string();
    let cur_time: &str = &Local::now().format("%H:%M").to_string();

    let post_dir_str: &str = &env::var("POST_DIR").unwrap();
    let post_dir: &Path = Path::new(&post_dir_str);

    let today_file_name: String = format!("{}.txt", today);
    let today_file_path: &Path = &post_dir.join(today_file_name);

    if !today_file_path.is_file(){
        let template_path_str: &str = &env::var("TEMPLATE").unwrap();
        let template_path: &Path = Path::new(&template_path_str);
        if let Err(e) = fs::copy(template_path, today_file_path) {
            println!("ERROR: Something went wrong creating new file");
            println!("ERROR: {:?}", e);
        }
    }

    let mut today_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(today_file_path)
        .unwrap();

    let msg_head: String = "-".repeat(37);
    let ts: String = format!("{}: ", cur_time);
    let msg_foot: String = "\n".to_owned();

    let mut temp = TinyTemplate::new();
    temp.add_template("message", TEMPLATE)?;

    let context = Context {
        message: msg,
        header: msg_head,
        footer: msg_foot,
        timestamp: ts,
    };

    let formatted_msg = temp.render("message", &context)?;

    if let Err(e) = writeln!(today_file, "{}", formatted_msg) {
        println!("ERROR: Failed to write to {:?}", today_file_path);
        println!("ERROR: {:?}", e);
    }

    Ok(())
}
