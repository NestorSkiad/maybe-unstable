use rss::Channel;
use std::error::Error;
use std::hash::Hasher;
use std::io::BufReader;
use ureq;
use ahash::AHasher;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut response =
        ureq::get("https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml").call()?;

    let chn = Channel::read_from(BufReader::new(response.body_mut().as_reader()))?;
    if chn.items.is_empty() {
        println!("No items found!");
    }

    for item in chn.items.into_iter() {
        //println!("{}", item.guid().unwrap().value());
        println!("{}", item.title().unwrap());
        let mut hasher = AHasher::default();
        let b = item.guid().unwrap().value().as_bytes();
        assert!(!b.is_empty());
        hasher.write(b);
        println!("Hash: {:?} \n", hasher.finish());
        //println!("end of item");
    }

    Ok(())
}
