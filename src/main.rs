use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient,
};

fn main() -> Result<(), > {
    let mut args = std::env::args();
    println!("{:?}", args);

    let mut _default_uri = "https://blogs.windows.com/feed";
    let arg_uri = args.nth(1).unwrap_or(_default_uri.to_string());

    let hs_arg_uri = HSTRING::from(arg_uri);
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
