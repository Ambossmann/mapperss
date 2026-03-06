use std::{mem::take, str::FromStr};

use atom_syndication::Feed;
use bpaf::*;
use mediatype::{MediaType, MediaTypeBuf, media_type, names::{APPLICATION, ATOM, RSS, XML}};
use reqwest::{Client, Response, header::HeaderValue};
use rss::Channel;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Options {}

#[derive(Debug)]
enum Syndication {
    RSS(Channel),
    Atom(Feed),
}

#[derive(Debug)]
enum FetchError {
    Reqwest(reqwest::Error),
    Parse(),
}

#[tokio::main]
async fn main() {
    let parsed_options = options().run();

    // let resp = reqwest::get("https://sample-feeds.rowanmanning.com/examples/18433b5bc1827086a07ba71f18dc9baa/feed.xml").await.unwrap();
    let synd = fetch_syndication("https://catfood.toolforge.org/?language=commons&project=wikimedia&category=Featured+pictures+on+Wikimedia+Commons&depth=0&namespace=6&user=&size=300&last=10&doit=Do+it")
        .await
        .unwrap();

    println!("{:#?}", synd);

    // println!("{:#?}", Channel::read_from(&resp.bytes().await.unwrap()[..]));
}

async fn fetch_syndication(url: &str) -> Result<Syndication, anyhow::Error> {
    let client = Client::builder()
        .user_agent("mapperss/0.0-pre-alpha")
        .build()?;

    println!("{:#?}", client.get(url).build()?);

    let mut resp = client.get(url).send().await?;

    println!("{:#?}", resp);

    const ATOM_MT: MediaType = media_type!(APPLICATION/ATOM+XML);
    const RSS_MT: MediaType = media_type!(APPLICATION/RSS+XML);

    // TODO: Test if taking out the headers breaks charset detection
    let resp_headers = take(resp.headers_mut());

    let content_type = resp_headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|h| HeaderValue::to_str(h).ok())
        .map(MediaType::parse)
        .and_then(Result::ok)
        .map(|mt| MediaType::from_parts(mt.ty, mt.subty, mt.suffix, &[]));

    let resp_text = resp.text().await;

    if let Some(mt) = content_type {
        println!("Atom: {}, RSS: {}", mt == ATOM_MT, mt == RSS_MT)
    }

    println!("{:#?}", resp_text);
    println!("{:#?}", Channel::from_str(resp_text.unwrap().as_str()));

    // println!("{:#?}");

    return Ok(Syndication::RSS(Channel::default()));
}
