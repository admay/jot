mod down;
mod create;

use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "jot", about = "Dead simple cli journal")]
enum Jot {
    #[structopt(name = "down")]
    Down {
        #[structopt(short, long, help = "Journal entry message")]
        message: String
    },
    #[structopt(name = "create")]
    Create {
        #[structopt(short, long, help = "Amount of med being taken")]
        dose: i8,
        #[structopt(short, long, help = "Wake up time")]
        wake: String,
        #[structopt(short, long, help = "Wake up mood")]
        mood: String
    }
}

fn main() -> std::io::Result<()>{
    match Jot::from_args() {
        Jot::Down { message } => {
            down::handle_post(message)?;
        },
        Jot::Create { dose, wake, mood } => {

        },
        _ => {
            println!("Invalid sub command");
        }
    }

    Ok(())
}
