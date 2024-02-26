use clap::Parser;
use tokio;
use reqwest;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args{
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "curl")]
    Curl(CurlCmd),
}

#[derive(Parser)]
struct CurlCmd {
    #[clap (short, long, value_parser)]
    path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        SubCommand::Curl(curl) => {
            let url = curl.path;
            match  get_request_body(&url).await {
                Ok(url) => println!("{:?}", url),
                Err(e) => println!("{:?}", e)
            }
        }
    }    
}

pub async fn get_request_body(req_url: &String) -> Result<String, reqwest::Error> {
    let content = reqwest::get(req_url).await?.text().await?;

    Ok(content)
}

