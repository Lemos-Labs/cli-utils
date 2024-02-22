use clap::Parser;
 mod ls_cmd;

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
            match ls_cmd::get_current_dir_entries(&current_dir) {
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