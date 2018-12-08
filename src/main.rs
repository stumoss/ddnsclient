extern crate clap;

use clap::{App, Arg, ArgMatches};

mod zoneedit;

pub fn main() {
    // Command line args take precedence over environment variables
    let cmd_args = parse_args();
    let domain = cmd_args.value_of("domain").unwrap_or("");
    let username = cmd_args.value_of("username").unwrap_or("");
    let password = cmd_args.value_of("password").unwrap_or("");

    match zoneedit::dns_update(&domain, &username, &password) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("showget")
        .version("0.0.1")
        .author("Stuart Moss <samoss@gmail.com>")
        .arg(
            Arg::with_name("domain")
                .short("d")
                .help("The domain to update the IP for")
                .env("DDNSCLIENT_DOMAIN")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("username")
                .short("u")
                .help(
                    "The username used to login to the dynamic DNS provider's website",
                )
                .env("DDNSCLIENT_USERNAME")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .help(
                    "The password used to login to the dynamic DNS provider's website",
                )
                .env("DDNSCLIENT_PASSWORD")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}
