mod interface;

use interface::*;

use prusti_contracts::*;

#[trusted]
fn pri(v: libc::c_long) {
    println!("{}", v);
}

fn main() {
    //prusti_inhale!(gpy_ref_held(1, PytObjectPointer { _private: 0 }));
    //prusti_exhale!(gpy_ref_held(1, PytObjectPointer { _private: 0 }));
    unsafe {
        pyt_initialize();
        let mut s = gpy_create_state();
        let lo = pytlong_fromlong(33, &mut s);
        if !lo.is_null() {
            pyt_decref(lo, &mut s);
        }
        prusti_refute!(false);
        pyt_finalize();
    }
    /*
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
    */
}
