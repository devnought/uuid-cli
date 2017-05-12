extern crate clap;
extern crate uuid;
extern crate clipboard;

mod cli;

use uuid::Uuid;
use clipboard::{ClipboardProvider, ClipboardContext};

fn main() {
    let uuid = Uuid::new_v4();
    let matches = cli::build_cli().get_matches();

    if matches.is_present(cli::CLIPBOARD) {
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().expect("Could not acquire a clipboard context");

        ctx.set_contents(uuid.hyphenated().to_string())
            .expect("Could not copy UUID to clipboard");
    }

    println!("{}", &uuid);
}
