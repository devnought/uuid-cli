use clap::{App, Arg};

pub const APP_NAME: &str = "uuid";
pub const CLIPBOARD: &str = "clipboard";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new(APP_NAME)
        .version("0.1")
        .author("Kyle Gretchev")
        .about("Generate UUIDs")
        .arg(Arg::with_name(CLIPBOARD).short("c").long(CLIPBOARD))
}
