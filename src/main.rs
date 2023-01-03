use structopt::StructOpt; // command line argument parser
use std::path::PathBuf;
use std::fs;


#[derive(StructOpt)]
#[structopt(name = "lsrs")]
struct Ls {
    #[structopt(default_value = "." ,name = "PATH", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Ls::from_args();
    let dir = &args.path;
    if dir.is_dir() {
		for e in fs::read_dir(dir).unwrap() {
			let file_name = e.unwrap().file_name();
            println!("{:?}", file_name)	
		}
	}
}