use clap::{App, Arg, AppSettings};

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main(){
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("config")
            .about("Config")
            .arg(Arg::new("url")
                 .short('u')
                 .long("url")
                 .required(true)
                 .takes_value(true))
            .short('c')
            .long("config")
            .takes_value(false))
        .get_matches();

}
