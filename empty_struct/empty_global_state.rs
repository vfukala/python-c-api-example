use prusti_contracts::*;

struct PyGlobalState {
}

#[trusted]
#[pure]
fn get_gcount(s: &PyGlobalState) -> i32 {
    1
}

#[trusted]
#[ensures(get_gcount(s) == old(get_gcount(s)) + 1)]
fn inc_count(s: &mut PyGlobalState) {
}

fn main() {
    let mut s = PyGlobalState { };
    inc_count(&mut s);
    assert!(false); // this verifies !!!
}
