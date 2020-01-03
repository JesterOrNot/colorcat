use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(default_value = "txt")]
    language: String,
}

fn main() {
    let args = Cli::from_args();
    let file_content = std::fs::read_to_string(&args.path);
    if let Err(_) = file_content {
        println!("\x1b[1;31mError File {:?} Not Found\x1b[0m", &args.path);
        std::process::exit(0);
    } else {
        if &args.language != "txt" {
            return;
        } else {
            println!("{}", file_content.unwrap());
        }
    }
}
