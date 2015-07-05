extern crate regex;
extern crate hyper;

use self::hyper::Client;
use self::hyper::header::{Headers, Authorization, Basic};
use self::regex::Regex;
use std::io::Read;

static UPDATE_URL: &'static str = "https://dynamic.zoneedit.com/auth/dynamic.html";

// Update the DNS entry
pub fn dns_update(domain: &str, username : &str, password : &str) -> Result<String, String> {
    let mut headers = Headers::new();

    headers.set(
        Authorization(
            Basic {
                username: username.to_string(),
                password: Some(password.to_string())
            }
            )
        );

    let full_www_url = format!("{}?zones={}", UPDATE_URL, domain);
    let client = Client::new();
    let mut response = client.get(&full_www_url)
                             .headers(headers)
                             .send()
                             .unwrap();

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    parse_return_code(&body)
}

// Parse the return code
fn parse_return_code(response: &str) -> Result<String, String> {
    let re = Regex::new("<\\w.*=\"(\\d+)\"\\s+TEXT=\"(\\w[^\"]*)\"\\s+ZONE=\"([^\"]*)\">").unwrap();
    let cap = match re.captures(&response) {
        Some(cap) => cap,
        None => return Err(format!("{}", response)),
    };

    let return_code = cap.at(1).unwrap().parse();
    match return_code {
        Ok(200...201) => return Ok(format!("{}", cap.at(2).unwrap())),
        Ok(701...705) => return Ok(format!("{}", cap.at(2).unwrap())),
        Ok(707...708) => return Ok(format!("{}", cap.at(2).unwrap())),
        _ => return Err(format!("{}", cap.at(2).unwrap())),
    }
}
