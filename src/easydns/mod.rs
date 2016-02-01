#![feature(phase)]
#[phase(plugin)]
extern crate regex;
extern crate regex_macros;
extern crate curl;
extern crate serialize;

use curl::http;
use serialize::base64::{ToBase64, STANDARD};

static full_www_url: &'static str = format!("{}{}",
                                            "http://members.easydns.com/dyn/dyndns.php?hostname=",
                                            domain);

// Update the DNS entry
fn dns_update(domain: &'static str,
              username: &'static str,
              password: &'static str)
              -> Result<String, String> {
    let authentication_string = format!("{}:{}", username, password).as_bytes().to_base64(STANDARD);

    let response = match http::handle()
                             .post(full_www_url.as_slice(), "")
                             .header("Authorization",
                                     format!("Basic {}", authentication_string).as_slice())
                             .exec() {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    parse_return_code(std::str::from_utf8(response.get_body()).unwrap());
}

// Parse the return code
fn parse_return_code(response: &str) -> Result<String, String> {
    let re = regex!(r"<\w.*=\W(\d+)\W\s+TEXT=\W(\w.*)\.\W");
    let cap = re.captures(response.as_slice()).unwrap();

    let return_code: Option<int> = from_str(cap.at(1));
    match return_code {
        Some(200...201) => return Ok(format!("{}", cap.at(2))),
        Some(701...705) => return Ok(format!("{}", cap.at(2))),
        Some(707...708) => return Ok(format!("{}", cap.at(2))),
        Some(_) => return Err("Unrecognised return code".to_string()),
        None => return Err(format!("Unable to convert {} to integer", cap.at(2))),
    }
}
