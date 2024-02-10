use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient,
};

fn main() -> Result<()> {
    let _uri = Uri::CreateUri(h!("https://blogs.windows.com/feed"))?;
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
