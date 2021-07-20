
#[macro_use] extern crate prettytable;
extern crate select;
extern crate reqwest;
extern crate tokio;
extern crate futures;

use std::fs;
use std::error::Error;
use std::io;
use select::document::Document;
use std::io::prelude::*;
use futures::executor::block_on;
use select::predicate::{Class, Attr, Predicate, Name};

/// fetches data from given site and stores it to temporary file.
pub async fn fetchDataSite(url: &str, dir_name:&str, filename:&str) -> Result<String, Box<dyn Error>> {


    let mut file_output =  fs::File::create(&format!("{}/{}", dir_name, filename)).expect("File created");
    let mut content = reqwest::get(url).await.expect("request failed");
    let mut readable_content = content.text().await.expect("Nothing");


    io::copy(&mut readable_content.as_bytes(), &mut file_output).expect("nothing");
    Ok(format!("{}/{}", dir_name,filename))
}

pub fn searchNews(path: &str, search_news: &str) {
    let document = Document::from(include_str!("webfile.txt"));
    // search for node where the table is with news
    let mut mysearch = String::new();
    let mut myurl = String::new();
    for node in document.find(Class("m-title-with-label-item")) {
        if node.text().to_lowercase().contains(&search_news.to_lowercase()) {
                mysearch = node
                .find(Class("m-title-with-label-item__title"))
                .next()
                .unwrap()
                .text();
	// take the url adres
            for myurls in node.attrs(){

                if myurls.0.contains("data-vr-contentbox-url"){
                    myurl = myurls.1.to_string();
                }
            }

        }
    }
    let table = table!([mysearch,myurl]);
    table.printstd();
}

#[tokio::main]
async fn main() {
    let url = "https://www.bankier.pl";
    let dir_name = "/Documents";
    let filename = "webfile.txt";

    let handle = tokio::runtime::Handle::current();
    handle.enter();
    let path =  block_on(fetchDataSite(url, dir_name,filename)).expect("File not processed");
    let search_news ="Rosja";
    searchNews(&path, &search_news);
}


