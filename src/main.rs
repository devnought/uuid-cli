use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(StructOpt, Debug)]
#[structopt(name = "uuid")]
struct Opts {
    /// Copy generated UUID to clipboard
    #[structopt(short = "c", long = "clipboard")]
    clipboard: bool,
}

fn main() {
    let opts = Opts::from_args();
    let uuid = Uuid::new_v4().to_hyphenated();

    if opts.clipboard {
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().expect("Could not acquire a clipboard context");

        ctx.set_contents(uuid.to_string())
            .expect("Could not copy UUID to clipboard");
    }

    println!("{}", &uuid);
}
