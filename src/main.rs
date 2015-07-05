extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::env;
use std::process;

// The domain to update
static DOMAIN_ENV   : &'static str = "DDNSCLIENT_DOMAIN";
// Your DDNS provider username
static USERNAME_ENV : &'static str = "DDNSCLIENT_USERNAME";
// Your DDNS provder password
static PASSWORD_ENV : &'static str = "DDNSCLIENT_PASSWORD";

mod zoneedit;

pub fn main() {
    let mut domain = String::new();
    let mut username = String::new();
    let mut password = String::new();

    // Command line args take precedence over environment variables
    let cmd_args = parse_args();
    domain.push_str(cmd_args.value_of("domain").unwrap_or(""));
    username.push_str(cmd_args.value_of("username").unwrap_or(""));
    password.push_str(cmd_args.value_of("password").unwrap_or(""));

    if domain == "".to_owned() {
        domain = get_env_arg(DOMAIN_ENV);
    }

    if username == "".to_owned() {
        username = get_env_arg(USERNAME_ENV);
    }

    if password == "".to_owned() {
        password = get_env_arg(PASSWORD_ENV);
    }

    match zoneedit::dns_update(&domain, &username, &password) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
}

fn get_env_arg(arg_name: &'static str) -> String {
    match env::var(arg_name) {
        Ok(var) => var,
        Err(_) => {
            println!("{} was not defined", arg_name);
            process::exit(1);
        },
    }
}

fn parse_args<'a, 'b>() -> ArgMatches<'a, 'b> {
    App::new("showget")
        .version("0.0.1")
        .author("Stuart Moss <samoss@gmail.com>")
        .arg(Arg::with_name("domain")
             .short("d")
             .help("The domain to update the IP for")
             .takes_value(true))
        .arg(Arg::with_name("username")
             .short("u")
             .help("The username used to login to the dynamic DNS provider's website")
             .conflicts_with("token")
             .takes_value(true))
        .arg(Arg::with_name("password")
             .short("p")
             .help("The password used to login to the dynamic DNS provider's website")
             .conflicts_with("token")
             .takes_value(true))
        .get_matches()
 }

