#![feature(phase)]
extern crate std;
extern crate curl;
extern crate serialize;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

#[phase(plugin, link)]
extern crate log;

use curl::http;
use serialize::base64::{ToBase64, STANDARD};

static domain_env   : &'static str = "DDNSCLIENT_DOMAIN";
static username_env : &'static str = "DDNSCLIENT_USERNAME";
static password_env : &'static str = "DDNSCLIENT_PASSWORD";

pub fn main() {

    let mut domain = String::new();
    match std::os::getenv(domain_env) {
        Some(dom) => domain = dom,
        None => { println!("{} unset", domain_env); () }
    };

    let mut username = String::new();
    match std::os::getenv(username_env) {
        Some(uname) => username = uname,
        None => { println!("{} unset", username_env); () }
    };

    let mut password = String::new();
    match std::os::getenv(password_env) {
        Some(pass) => password = pass,
        None => { println!("{} unset", password_env); () }
    };

    println!("domain   = {}", domain);
    println!("username = {}", username);
    println!("password = {}", password);

    println!("IP = {}", get_current_ip());
    let ip = get_current_ip();

    let full_www_url = format!("{}{}{}{}", "http://dynamic.zoneedit.com/auth/dynamic.html?host=*.", domain, "?dnsto", ip);

    let authentication_string = format!("{}:{}", username, password).as_bytes().to_base64(STANDARD);

    let resp = http::handle()
      .post(full_www_url.as_slice(), "")
      .header("Authorization", format!("Basic {}", authentication_string).as_slice())
      .exec().unwrap();

    println!("code={}; headers={}; body={}", resp.get_code(), resp.get_headers(), resp.get_body());
}

pub fn get_current_ip() -> String {
    let resp = match http::handle().get("http://checkip.dyndns.org").exec() {
        Ok(resp) => resp,
        Err(e) => { println!("ddnsclient: {}", e); return "".to_string(); },
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
