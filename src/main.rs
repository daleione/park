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
                .long("file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .required(false)
                .help("Add prefix before the key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("upper_case")
                .short("u")
                .long("upper_case")
                .help("Upper case the key")
                .required(false)
        )
        .get_matches();
    let config = matches.value_of("config").unwrap();
    let env_file = matches.value_of("env_file").unwrap();
    let upper_case = matches.is_present("upper_case");
    let prefix = matches.value_of("prefix").unwrap_or("");
    let conf = park::Config::new(config, env_file, upper_case, prefix);

    park::run(conf);
}
