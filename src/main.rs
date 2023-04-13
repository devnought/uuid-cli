use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Copy generated UUID to clipboard
    #[arg(short, long)]
    clipboard: bool,
}

fn main() {
    let opts = Args::parse();
    let uuid = Uuid::new_v4();

    if opts.clipboard {
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().expect("Could not acquire a clipboard context");

        ctx.set_contents(uuid.to_string())
            .expect("Could not copy UUID to clipboard");
    }

    println!("{uuid}");
}
