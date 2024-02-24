use clap::Parser;
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "https://blogs.windows.com/feed")]
    uri: String,
}

fn main() -> Result<(), > {
    println!("{:?}", std::env::args());
    let args = Cli::parse();

    let hs_arg_uri = HSTRING::from(args.uri);
    let _uri = Uri::CreateUri(&hs_arg_uri)?;
    let _client = SyndicationClient::new()?;

    _client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)"),
    )?;
    
    let _feed = _client.RetrieveFeedAsync(&_uri)?.get()?;

    println!("Hello, world!");
    for item in _feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }
    
    Ok(())
}
