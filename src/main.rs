use rss::Channel;
use std::error::Error;
use std::hash::Hasher;
use std::io::BufReader;
use ureq;
use ahash::AHasher;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut response =
        ureq::get("https://rss.nytimes.com/services/xml/rss/nyt/recent.xml").call()?;

    let chn = Channel::read_from(BufReader::new(response.body_mut().as_reader()))?;
    if chn.items.is_empty() {
        println!("No items found!");
    }

    for item in chn.items.into_iter() {
        println!("{}", item.guid().unwrap().value());
        println!("{}", item.title().unwrap());
        let mut hasher = AHasher::default();
        hasher.write(item.guid().unwrap().value().as_bytes());
        println!("{}\n", hasher.finish());
    }

    Ok(())
}
