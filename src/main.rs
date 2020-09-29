extern crate reqwest;
extern crate scraper;
extern crate regex;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, ErrorKind};
use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
struct Payload {
    payload_type: String,
    payload_text: String,
}

fn payload_scraper<'a>(url: &String) -> Vec<&'a str> {

    // creating reqwest crate client
    let client = reqwest::blocking::Client::new();
    // assigning method url
    let input_url = url;

    let mut res = client.get(input_url).send().unwrap();

    println!("RUST WEB SCRAPER - Status for {}: {}", input_url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let mut code_txt = Vec::new();
    let mut payload_vector = Vec::new();
    let fragment = Html::parse_document(&body);
    let code_selector = Selector::parse("code").unwrap();

    for code_reference in fragment.select(&code_selector) {
        code_txt = code_reference.text().collect::<Vec<&str>>();
        payload_vector.push(code_txt[0]);
    }
    payload_vector
}

fn main() -> std::io::Result<()> {

    let mut input_url = String::new();

    println!("Enter URL for Scraper: ");

    io::stdin()
        .read_line(&mut input_url)
        .expect("Failed to read line");


    OpenOptions::new().write(true).create(true).open("payload.json");

    let data = fs::read_to_string("payload.json").expect("Unable to Read File!");
    let mut payload_from_method = payload_scraper(&input_url);
    let mut payload_holder: Vec<Payload> = Vec::new();

    for code in payload_from_method.iter() {
        let mut payload = Payload {
            payload_type: "1".to_string(),
            payload_text: code.to_string(),
        };
        payload_holder.push(payload);
    }

    if fs::metadata("payload.json").unwrap().len() != 0 {
       payload_holder = serde_json::from_str(&data)?;
    }

    let json: String = serde_json::to_string(&payload_holder)?;

    fs::write("payload.json", &json).expect("Unable to write file");

    println!("{}", &json);

    Ok(())
}