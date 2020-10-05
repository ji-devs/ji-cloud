use structopt::StructOpt;
use std::env;
use std::path::{Path, PathBuf};
use config::RemoteTarget;


#[derive(Debug, StructOpt)]
#[structopt(name = "database migrations", about = "A little util to run database migrations")]
pub struct Opts {
    // local, sandbox, or release 
    #[structopt(short, long)]
    pub remote_target: String,

    // project dir 
    #[structopt(short, long, required_unless="all-projects")]
    pub project: Option<String>,

    // show output 
    #[structopt(short, long)]
    pub verbose: bool,

    // clean before building 
    #[structopt(short, long)]
    pub clean: bool,

    // include storybook/demo-templates dir 
    #[structopt(short, long)]
    pub demo: bool,

    // show output 
    #[structopt(short, long)]
    pub all_projects: bool,
}

impl Opts {
    pub fn get_remote_target(&self) -> RemoteTarget {

        match self.remote_target.as_ref() {  
            "local" => RemoteTarget::Local,
            "sandbox" => RemoteTarget::Sandbox,
            "release" => RemoteTarget::Release,
            _ => panic!("target must be local, sandbox, or release")
        }
    }
    pub fn get_base_template_path(&self) -> PathBuf {
        let path = self.get_frontend_path().join("templates");

        if !path.exists() {
            panic!("base template path does not exist!");
        }

        path

    }
    pub fn get_common_template_path(&self) -> PathBuf {
        let path = self.get_base_template_path().join("_common");

        if !path.exists() {
            panic!("core template path does not exist!");
        }

        path

    }

    pub fn get_project_template_path(&self) -> PathBuf {
        let project = self.project.as_ref().unwrap();
        let path = self.get_base_template_path().join(&project);
        if !path.exists() {
            panic!("template path for [{}] does not exist!", &project);
        }

        path
    }


    pub fn get_demo_template_path(&self) -> PathBuf {

        let path = self.get_frontend_path().join("storybook").join("demo-templates");

        if !path.exists() {
            panic!("demo template path does not exist!");
        }

        path
    }

    pub fn get_project_output_path(&self) -> PathBuf {
        self.get_frontend_path().join(".template_output")
    }


    fn get_frontend_path(&self) -> PathBuf {
        Path::new(&env::var("LOCAL_CDN_FRONTEND_DIR").expect("needs env var LOCAL_CDN_FRONTEND_DIR")).to_path_buf()
    }

}


