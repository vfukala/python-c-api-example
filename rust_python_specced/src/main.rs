mod interface;

use interface::*;

use prusti_contracts::*;

#[trusted]
fn pri(v: libc::c_long) {
    println!("{}", v);
}

fn main() {
    unsafe {
        Py_Initialize();
        let lo = pytlong_fromlong(33);
        if !lo.is_null() {
            pyt_incref(lo);
            pyt_incref(lo);
            pyt_decref(lo);
            pyt_decref(lo);
            pri(pytlong_aslong(lo)); // prints 33
            pyt_decref(lo);

        }
        Py_Finalize();
    }
}
