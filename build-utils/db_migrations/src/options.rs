use structopt::StructOpt;

#[derive(PartialEq, Eq)]
pub enum Target {
    Local,
    Sandbox,
    Release
}

pub fn get_target(target_str:&str) -> Target {

    match target_str {  
        "local" => Target::Local,
        "sandbox" => Target::Sandbox,
        "release" => Target::Release,
        _ => panic!("target must be local, sandbox, or release")
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "database migrations", about = "A little util to run database migrations")]
pub struct Opts {
    // local, sandbox, or release 
    #[structopt(short, long)]
    pub target: String,

    // show output 
    #[structopt(short, long)]
    pub verbose: bool,

    // show connection string only 
    #[structopt(short, long)]
    pub connection_string_only: bool,
}


