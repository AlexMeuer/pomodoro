use std::process::ExitCode;

mod shell;
mod timer;

fn main() -> ExitCode {
    match shell::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
