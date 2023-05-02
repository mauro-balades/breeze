use std::collections::HashMap;

use label_logger::warn;
use subprocess::{Exec, Redirection};

use crate::{Args, runner::Runner};


pub fn exec(argv: HashMap<String, String>, args: Args, runner: &mut Runner) -> () {
    if !argv.contains_key("cmd") {
        Runner::throw_error("'exec' function requires the 'cmd' parameter!".to_string());
    }

    let cmd = argv.get("cmd").unwrap();
    if args.debug {
        warn!(label: "[Debug]:", "Executing '{}' as a command from shell", cmd);
    }

    let out = Exec::shell(cmd)
        .stdout(Redirection::Pipe)
        .capture();


    if out.is_err() {
        if argv.contains_key("stderr") {
            runner.generate_variable(argv.get("stderr").unwrap().to_owned(), &out.unwrap_err().to_string());
        }

        if argv.contains_key("success") {
            runner.generate_variable(argv.get("success").unwrap().to_owned(), &"0".to_string());
        }
    } else {
        let o = out.unwrap();
        if argv.contains_key("stdout") {
            runner.generate_variable(argv.get("stdout").unwrap().to_owned(), &o.stdout_str());
        }

        if argv.contains_key("stderr") {
            runner.generate_variable(argv.get("stderr").unwrap().to_owned(), &o.stderr_str());
        }

        if argv.contains_key("success") {
            runner.generate_variable(argv.get("success").unwrap().to_owned(), &o.success().to_string());
        }
    }
}

pub fn get_std_functions() -> HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()> {
    let mut m: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()> = HashMap::new();

    m.insert("exec".to_string(), exec);
    return m;
}
