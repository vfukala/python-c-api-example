use prusti_contracts::*;

struct PyGlobalState {
    c: i32
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
    let mut s = PyGlobalState { c: 0 };
    inc_count(&mut s);
    assert!(false); // this fails to verify (good)
}
