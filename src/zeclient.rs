#![feature(phase)]
extern crate std;
extern crate curl;
extern crate serialize;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

#[phase(plugin, link)] extern crate log;

use curl::http;
use serialize::base64::{ToBase64, STANDARD};

static domain_env   : &'static str = "ZECLIENT_DOMAIN";
static username_env : &'static str = "ZECLIENT_USERNAME";
static password_env : &'static str = "ZECLIENT_PASSWORD";

pub fn main() {
    let mut domain = String::new();
    match std::os::getenv(domain_env) {
        Some(dom) => domain = dom,
            None => { error!("{} unset", domain_env); () }
    };

    let mut username = String::new();
    match std::os::getenv(username_env) {
        Some(uname) => username = uname,
            None => { error!("{} unset", username_env); () }
    };

    let mut password = String::new();
    match std::os::getenv(password_env) {
        Some(pass) => password = pass,
            None => { error!("{} unset", password_env); () }
    };

    let full_www_url = format!("{}{}", "https://dynamic.zoneedit.com/auth/dynamic.html?host=*.", domain);
    let authentication_string = format!("{}:{}", username, password).as_bytes().to_base64(STANDARD);

    let response = match http::handle()
        .post(full_www_url.as_slice(), "")
        .header("Authorization", format!("Basic {}", authentication_string).as_slice())
        .exec() {
            Ok(response) => response,
                Err(e) => { error!("{}", e); return },
        };

    parse_return_code(std::str::from_utf8(response.get_body()).unwrap());
}

fn parse_return_code(response: &str) {
    let re = regex!(r"<\w.*=\W(\d+)\W\s+TEXT=\W(\w.*)\.\W");
    let cap = re.captures(response.as_slice()).unwrap();

    let return_code: Option<int> = from_str(cap.at(1));
    match return_code {
        Some(200..201) => println!("{}", cap.at(2)),
        Some(701..705) => println!("{}", cap.at(2)),
        Some(707..708) => println!("{}", cap.at(2)),
        Some(_) => println!("Unrecognised return code"),
        None => println!("Unable to convert {} to integer", cap.at(2)),
    }
}
