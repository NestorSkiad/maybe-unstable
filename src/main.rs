use rss::Channel;
use std::error::Error;
use std::hash::Hasher;
use std::io::BufReader;
use ahash::AHasher;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let sample_data = include_str!("../sample input.xml");
    let chn = Channel::read_from(BufReader::new(sample_data.as_bytes()))?;
    if chn.items.is_empty() {
        println!("No items found!");
    }

    for item in chn.items.into_iter() {
        println!("{:?}", item.title());
        let mut hasher = AHasher::default();
        hasher.write(item.link().unwrap().as_bytes());
        println!("Hash: {:?}\n", hasher.finish()); // should fail here
    }

    Ok(())
}
