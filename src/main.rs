extern crate clap;
extern crate park;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Park")
        .version("1.0")
        .author("Dalei. <guoyunlei@live.com>")
        .about("Convert toml to docker env file")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("env_file")
                .short("f")
                .required(true)
                .long("file")
                .takes_value(true),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap();
    let env_file = matches.value_of("env_file").unwrap();
    let conf = park::Config::new(config, env_file);

    park::run(conf);
}
