mod as_ref;
pub mod borrow;
mod copy_clone;
mod to_owned;

pub fn main() {
    borrow::main();
    as_ref::main();
    to_owned::main();
    copy_clone::main();
}
