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

    // Need to save the old ip address and compare it here to see if it's
    // changed
    //if get_current_ip() !=

    let response = match http::handle()
        .post(full_www_url.as_slice(), "")
        .header("Authorization", format!("Basic {}", authentication_string).as_slice())
        .exec() {
        Ok(response) => response,
        Err(e) => { error!("{}", e); return },
    };

    println!("code={}; headers={}; body={}", response.get_code(), response.get_headers(), std::str::from_utf8(response.get_body()));
    //parse_return_code(std::str::from_utf8(response.get_body()).unwrap());
}

/*
fn parse_return_code(response: &str) -> bool {
    let re = regex!(r"<SUCCESS CODE='(\d{3})' TEXT='(\w+)'\.");
    let cap = re.captures(response.as_slice()).unwrap();

    println!("Response Code= {}, Text= {}", cap.at(1), cap.at(2));
    return true;
}
*/

fn get_current_ip() -> String {
    let resp = match http::handle().get("http://checkip.dyndns.org").exec() {
        Ok(resp) => resp,
        Err(e) => { error!("{}", e); return "".to_string(); },
    };

    match resp.get_code() {
        200 => (),
        _ => { error!("Failed to retrieve current public IP"); return "".to_string() }
    }

     let re = regex!(r"([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})");
     let response_string = std::str::from_utf8(resp.get_body()).unwrap();

     let cap = re.captures(response_string).unwrap();
     let ip = format!("{}.{}.{}.{}", cap.at(1), cap.at(2), cap.at(3), cap.at(4));

    return ip.clone();
}
