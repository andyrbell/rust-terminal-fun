#[macro_use]
extern crate serde_derive;
extern crate hyperlocal;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;


#[allow(unused_imports)]
use crate::docker::hello_docker;
use structopt::StructOpt;

pub mod docker;
pub mod github;
pub mod ui; 
pub mod devoxx;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-web", about = "A command line tool")]
struct Opt {
    #[structopt(short, long)]
    debug: bool
}

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    ui::repos_screen::run()
}
