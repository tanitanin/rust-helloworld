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

#[derive(Debug)]
enum CliError {
    NotEnoughArgs,
    Parse(std::num::ParseIntError),
}
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::NotEnoughArgs => write!(f, "Not enough arguments."),
            CliError::Parse(ref e) => write!(f, "Parse error: {}", e),
        }
    }
}
impl std::error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::NotEnoughArgs => "Not enough arguments.",
            CliError::Parse(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
impl From<CliError> for Error {
    fn from(value: CliError) -> Self {
        match value {
            CliError::NotEnoughArgs => Error::new(HRESULT(-1), From::from("Not enough arguments.")),
            CliError::Parse(e) => Error::new(HRESULT(-2), From::from(e.to_string())),
        }
    }
}

fn double_arg(arg: &str) -> std::result::Result<i32, CliError> {
    let n = arg.parse::<i32>().map_err(|e| CliError::Parse(e))?;
    Ok(2 * n)
}

#[test]
fn check_double_arg_validity() {
    assert_eq!(double_arg("100").unwrap(), 200);
}

fn main() -> Result<(), > {
    println!("{:?}", std::env::args());
    let args = Cli::parse();

    let val = "1234321";
    let double_val = double_arg("1234321")?;
    println!("{} * 2 = {}", val, double_val);

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
