#[macro_use]
extern crate serde_derive;
extern crate tinytemplate;

mod create;
mod down;
mod view;

use std::env;
use dotenv::dotenv;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "jot", about = "Dead simple cli journal")]
enum Jot {
    #[structopt(name = "down")]
    Down {
        #[structopt(short, long, help = "Journal entry message")]
        message: String,
    },
    #[structopt(name = "create")]
    Create {
        #[structopt(short, long, help = "Amount of med taken")]
        dose: i8,
        #[structopt(short, long, help = "Wake up time")]
        wake: String,
        #[structopt(short, long, help = "Wake up mood")]
        mood: String,
    },
    View {
        #[structopt(short, long, help = "Date of entry to see")]
        date: String,
    },
    Conf {
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    match Jot::from_args() {
        Jot::Down { message } => {
            down::handle(message)?;
        }
        Jot::Create { dose, wake, mood } => {
            create::handle(dose, wake, mood)?;
        },
        Jot::View { date } => {
            view::handle(date)?;
        },
        Jot::Conf {} => {
            for (k, v) in env::vars() {
                println!("{}:{}", k, v);
            }
        },
    }

    Ok(())
}
