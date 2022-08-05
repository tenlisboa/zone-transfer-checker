use execute::Execute;
use std::process::{Command, Stdio};

// fn get_name_servers(domain: String) {}

fn capture_output(command: &mut Command) {
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
}

fn run_command(program: &str, args: &[&str]) -> Command {
    let mut command = Command::new(program);
    command.args(args);

    return command;
}

fn extract_output(command: &mut Command) -> String {
    let output = command.execute_output().unwrap();

    return String::from_utf8(output.stdout).unwrap();
}

fn run(args: &[&str]) -> String {
    let mut which_command = run_command(args[0], &args[1..]);

    capture_output(&mut which_command);

    return extract_output(&mut which_command);
}

fn main() {
    print!("{}", run(&["host", "-t", "ns", "blockforce.in"]));
}
