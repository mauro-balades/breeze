
use crate::langs::Language;
use std::collections::HashMap;
use crate::runner::Runner;
use crate::Args;
use subprocess::{Exec};
use label_logger::{warn, error};

fn check_device(args: HashMap<String, String>, cli: Args, runner: &mut Runner) {
    Runner::check_arguments(&args, HashMap::from([]), "@python.check_device");

    if cli.debug {
        warn!(label: "[Debug]:", "Checking if device has python and pip installed");
    }

    // TODO: allow version 2 too!

    // check if python and pip are installed
    let python = Exec::shell("python3 --version")
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Pipe)
        .capture()
        .unwrap();
    let pip = Exec::shell("pip3 --version")
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Pipe)
        .capture()
        .unwrap();
    if !python.success() || !pip.success() {
        Runner::throw_error("Python 3 and/or pip3 are not installed!".to_string());
    }
}

fn install_requirements(args: HashMap<String, String>, cli: Args, runner: &mut Runner) {
    Runner::check_arguments(&args, HashMap::from([("file", false)]), "@python.install_requirements");

    if cli.debug {
        warn!(label: "[Debug]:", "Installing requirements");
    }

    // install requirements
    let default_file = "requirements.txt".to_string();
    let requirements = args.get("file").unwrap_or(&default_file);
    let pip = Exec::shell(format!("pip install -r {}", requirements))
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Pipe)
        .capture()
        .unwrap();
    if !pip.success() {
        Runner::throw_error(format!("Failed to install requirements from {}\n{}", requirements, pip.stderr_str()));
    }
}


pub fn get_python() -> Language {
    let mut lang = Language {
        name: "python".to_string(),
        functions: HashMap::new(),
    };
    lang.functions.insert("check_device".to_string(), check_device);
    lang.functions.insert("install_requirements".to_string(), install_requirements);
    lang
}
