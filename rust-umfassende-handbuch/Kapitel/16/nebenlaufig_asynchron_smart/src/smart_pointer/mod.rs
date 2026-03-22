#![cfg(feature = "smart_pointer")]

mod cow;
mod cell;
mod refcell;
mod oncecell;
mod rc;

pub fn main() {
    cow::main();
    cell::main();
    refcell::main();
    oncecell::main();
    rc::main();
}