use std::env;

// Will eventually expand this to have subcommands
// jot open -> make a new file for today
// jot down -> write a message
// jot publish -> commit and push
fn main() {
    let args: Vec<String> = env::args().collect();
    let msg: &[String] = &args[1..];
    println!("Hello, world!");
    println!("{:?}", msg);

    // todays_date
    // if file exists:
    //     append to file
    // else:
    //     create file and append
    //
    // Find today's file
    // Append the new line, a timestamp, and some formatting
}
