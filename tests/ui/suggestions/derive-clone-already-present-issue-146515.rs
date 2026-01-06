// issue: https://github.com/rust-lang/rust/issues/146515

use std::rc::Rc;

#[derive(Clone)]
struct ContainsRc<T> {
    value: Rc<T>,
}

fn clone_me<T>(x: &ContainsRc<T>) -> ContainsRc<T> {
    x.clone()
    //~^ ERROR mismatched types
}

fn main() {}
