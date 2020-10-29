use std::env;
use std::fs;
use std::fs::{OpenOptions};
use std::path::Path;
use std::io::prelude::*;
use chrono::prelude::*;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let msg: String = args[1..].to_owned().join(" ");
    println!("Hello, world!");
    println!("{:?}", msg);

    let today: &str = &Local::today().format("%Y-%b-%d").to_string();
    let cur_time: &str = &Local::now().format("%H:%M").to_string();
    println!("{:?}", today);

    // should be a path
    let journal_dir : &Path = Path::new("/home/michael/workspace/mental-health-home-base/ihaveadhd.github.io/days");
    let today_file_name: String = format!("{}.txt", today);
    let today_file_path: &Path = &journal_dir.join(today_file_name);
    let template_path: &Path = &journal_dir.join("template.txt");

    println!("{:?}", today_file_path);

    if !today_file_path.is_file(){
        println!("No file for {} found", today);
        println!("Creating one from template");
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

    let msg_head = "-".repeat(37);
    let ts = format!("{}: ", cur_time);
    let msg_foot = "\n";

    let formatted_msg = format!("{}\n\n{}{}\n{}", msg_head, ts, msg, msg_foot);

    if let Err(e) = writeln!(today_file, "{}", formatted_msg) {
        println!("ERROR: Failed to write to {:?}", today_file_path);
        println!("ERROR: {:?}", e);
    }

    Ok(())
}
