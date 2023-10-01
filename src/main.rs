//use std::ffi::CString;
//use std::thread::sleep;
//use std::time::Duration;

use std::fs::read_to_string;
use clap::{Arg, command};
use git2::{Deltas, DiffOptions};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Configuration {
    modules: Vec<ModuleConfiguration>
}

#[derive(Serialize, Deserialize)]
struct ModuleConfiguration {
    name: String,
    path: String,
    technology: String
}

fn main() {
    pretty_env_logger::init_timed();
    let matches = command!() // requires `cargo` feature
        .name("gitmnr")
        .version("1.0")
        .about("simplify git monorepo operations")
        .arg(Arg::new("repo").short('r').long("repo").help("git repository path (default to current folder)").default_value("."))
        .arg(Arg::new("target_branch_or_tag").short('t').long("target").help("target branch or tag name or any revision spec (default to origin/master)").default_value("origin/master"))
        .arg(Arg::new("module").short('m').long("module").help("module name").required(true))
        .get_matches();

    let repo_path = matches.get_one::<String>("repo").unwrap();
    let config_text = read_to_string(format!("{repo_path}/gitmnr.json")).unwrap();
    let module_name = matches.get_one::<String>("module").unwrap().to_string();
    let config: Configuration = serde_json::from_str(&config_text).unwrap();
    match config.modules.iter().find(|module| {
        return module.name == module_name
    }) {
        Some(module_config) => {
            info!("start open repository {:?}", repo_path);
            //sleep( Duration::from_secs(10));
            let repo = match git2::Repository::open(repo_path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            };
            //sleep( Duration::from_secs(10));
            info!("end open repository");
            let rev_spec_arg = matches.get_one::<String>("target_branch_or_tag").unwrap();
            let rev_spec = if rev_spec_arg == "" { "origin/master" } else { rev_spec_arg };
            info!("start find master branch");
            let git_rev = repo.revparse(rev_spec).unwrap().to().unwrap().peel_to_tree().unwrap();
            info!("end find master branch");
            let mut diff_options = DiffOptions::new();
            diff_options.pathspec(&module_config.path);
            diff_options.patience(false); // we don't need "clean" diff
            info!("start diff");
            //sleep(Duration::from_secs(10));
            let diff = repo
                .diff_tree_to_workdir(
                    Some(&git_rev),
                    Some(&mut diff_options),
                )
                .unwrap();
            info!("end diff");
            //sleep( Duration::from_secs(10));
            info!("get stats");
            let stats = match diff.stats() {
                Ok(stats) => stats,
                Err(e) => panic!("failed to get diff stat: {}", e),
            };
            let changes = stats.files_changed() + stats.deletions() + stats.insertions();
            info!("there's {} changes", changes);
            let deltas = diff.deltas();
            match deltas.len() {
                0 => {
                    info!("there's no changes")
                }
                _ => {
                    process_deltas(deltas);
                }
            }
        }
        None => {
            panic!("failed to get configuration of module {module_name}");
        }
    }


}

fn process_deltas(deltas: Deltas) {
    deltas.for_each(|delta| {
        let new_file = delta.new_file();
        let path = new_file.path().unwrap().to_str().unwrap().to_owned();
        info!("{}", path);
    });
}
