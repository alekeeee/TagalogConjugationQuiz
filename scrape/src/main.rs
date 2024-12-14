use reqwest;
use scraper::{Selector, Html};
use tokio;
use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::to_writer_pretty;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("data.txt")?;
    let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
    let url = "https://seasite.niu.edu/tagalog/tagalog_verbs.htm";
 

    // Send a GET request to the website
    let response = reqwest::get(url).await?;

    // Check if request was successful
    if response.status().is_success() {
        let body_bytes = response.bytes().await?;
        let mut decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(&*body_bytes);
        let mut body = String::new();
        decoder.read_to_string(&mut body)?;

        let document = Html::parse_document(&body);
        // Define a selector for table rows (<tr>)
        let table_selector = Selector::parse("table").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let re = Regex::new(r"\s+").unwrap();

        // Extract verbs and their conjugations
        // need to do so in a way that only pulls the verbs and puts them in order.
        //Select alphabet a-z
        //then within each letter block, parse for tr/td

        for table in document.select(&table_selector).skip(1) {
            for row in table.select(&tr_selector).skip(3) {
                let columns: Vec<_> = row.select(&td_selector).collect();
                
                if columns.len() > 1{
                    let english_verb = re.replace_all(&columns[0]
                        .text()
                        .collect::<Vec<_>>()
                        .concat(), " ").trim()
                        .to_string();
                    let tagalog: Vec<String> = columns[1..]
                        .iter()
                        .map(|col| re.replace_all(&col.text().collect::<Vec<_>>().concat(), " ")
                        .replace("á", "a")
                        .replace("é", "e")
                        .replace("í", "i")
                        .replace("ó", "o")
                        .replace("ú", "u")
                        .replace("'", "")
                        .trim()
                        .to_string())
                        .collect();
                    dictionary.entry(english_verb).or_insert_with(|| tagalog);

                }
            }
 

    }

    // Send all entries inside dictionary into data.txt
    to_writer_pretty(&mut file, &dictionary)?;
}

    Ok(())
}


// .replace("á", "a")
