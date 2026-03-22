use std::process::ExitCode;

mod error_from;
mod gewicht_from;
mod into;
mod try_from_into;

pub fn main() {
    gewicht_from::main();
    // error_from::main().expect("");
    into::main();
    try_from_into::main();
}
