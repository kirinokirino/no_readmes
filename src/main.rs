use std::{
    error::Error,
    io::{self, BufRead, Write},
};

use reqwest::{blocking::Client, StatusCode};

fn main() {
    let mut no_readmes: Vec<String> = Vec::new();
    // if let Ok(urls) = urls() {
    let urls = include_str!("urls.txt");
    let client = Client::new();
    for url in urls.lines() {
        //println!("{url}");
        let req = client
            .get(url)
            .timeout(std::time::Duration::from_millis(20000))
            .build()
            .unwrap();
        let res = client.execute(req);
        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => print!("."),
                StatusCode::NOT_FOUND => {
                    print!("!");
                    no_readmes.push(url.to_string())
                }
                other => {
                    dbg!(other, &url);
                }
            },
            Err(err) => {
                dbg!(&err);
                //print!("!");
                //no_readmes.push(url.to_string())
            }
        }
        let _ = io::stdout().flush();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // }
    println!();
    for url in no_readmes {
        println!("{url}");
    }
}

fn urls() -> io::Result<Vec<String>> {
    let mut urls = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lines() {
        let line = line.expect("Could not read line from standard in");
        if let Some((path, _)) = line.split_once('\t') {
            let url = format!("https://github.com/{path}/blob/master/README.md");
            urls.push(url);
        }
    }
    Ok(urls)
}
