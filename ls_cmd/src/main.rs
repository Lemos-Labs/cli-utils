use clap::Parser;
use std::{fs, io, path::PathBuf};


 #[derive(Parser)]
 #[clap(author, version, about, long_about = None)]
 struct Args{
    #[clap(subcommand)]
    cmd: SubCommand,
 }

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "ls")]
    List(ListCmd),
}

#[derive(Parser)]
struct ListCmd {
    #[clap (short, long, value_parser)]
    directory_name: String,
}

 fn main(){
    let args = Args::parse();

    match args.cmd {
        SubCommand::List(list_cmd) => {
            let current_dir = list_cmd.directory_name;
            match get_current_dir_entries(&current_dir) {
                Ok(paths) => {
                    for path in paths {
                        println!("{}", path.display());
                    }
                }
                Err(e) => println!("Error:{}", e)
            }
        }
    }

 }

fn get_current_dir_entries(path: &String) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(path)?;

    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry?;
        paths.push(entry.path());
    };
    
    Ok(paths)
} 
