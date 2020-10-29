mod create;
mod down;

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
        #[structopt(short, long, help = "Amount of med being taken")]
        dose: i8,
        #[structopt(short, long, help = "Wake up time")]
        wake: String,
        #[structopt(short, long, help = "Wake up mood")]
        mood: String,
    },
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    match Jot::from_args() {
        Jot::Down { message } => {
            down::handle(message)?;
        }
        Jot::Create { dose, wake, mood } => {
            create::handle(dose, wake, mood)?;
        }
    }

    Ok(())
}
