use std::ffi::CString;
use std::io::{self, Write};
use prusti_contracts::*;

type Py_ssize_t = isize;

#[repr(C)]
#[derive(Clone, Copy)]
struct PyObjectPointer {
    _private: usize
}

struct PyGlobalState {
    _private: usize // add some data to convince Prusti that not all objects of this type are the
                    // same (?) (Prusti deduces unsoundness without this)
}


#[derive(Clone, Copy)]
enum PyObjectData {
    PyNone,
    PyLong,
    PyBool,
    PyDict,
    PyUnicode,
}

/*
#[derive(Clone, Copy)]
enum PyLongData {
    Known(libc::c_long),
    Unknown,
}
*/

impl PyObjectPointer {
    #[pure]
    fn is_null(&self) -> bool {
        self._private == 0
    }

    // use only in specifications
    #[trusted]
    #[pure]
    #[ensures(result ==> !self.is_null())]
    fn is_valid_pyobject(&self, s: &PyGlobalState) -> bool {
        // should be an abstract function
        true
    }

    // use only in specifications
    #[trusted]
    #[pure]
    #[requires(self.is_valid_pyobject(s))]
    fn get_data(&self, s: &PyGlobalState) -> PyObjectData {
        // should be an abstract function
        PyObjectData::PyNone
    }

    // use only in specifications
    #[trusted]
    #[pure]
    #[requires(self.is_valid_pyobject(s))]
    #[ensures(result >= 1)]
    fn get_refcount(&self, s: &PyGlobalState) -> Py_ssize_t {
        // should be an abstract function
        1
    }


    // use only in specifications
    #[trusted]
    #[pure]
    fn is_held_by_api(&self) -> bool {
        false
    }
}

type FfiString = *const libc::c_char;

extern "C" {
    fn Py_Initialize();
    // don't use directly
    fn Py_IncRef(obj: PyObjectPointer);
    // don't use directly
    fn Py_DecRef(obj: PyObjectPointer);
    // don't use directly
    fn Py_REFCNT(obj: PyObjectPointer) -> Py_ssize_t;

    // don't use directly
    //fn PyLong_AsLong(obj: PyObjectPointer) -> libc::c_long;
    // don't use directly
    fn PyLong_FromLong(v: libc::c_long) -> PyObjectPointer;
    // don't use directly
    fn MyLong_CheckExact(obj: PyObjectPointer) -> libc::c_int;

    // don't use directly
    fn PyDict_New() -> PyObjectPointer;
    // don't use directly
    fn MyDict_CheckExact(obj: PyObjectPointer) -> libc::c_int;

    // don't use directly
    fn MyNone_GetNoIncRef() -> PyObjectPointer;
    // don't use directly
    fn MyBool_GetTrueNoIncRef() -> PyObjectPointer;
    // don't use directly
    fn MyBool_GetFalseNoIncRef() -> PyObjectPointer;

    // don't use directly
    fn MyBool_Check(obj: PyObjectPointer) -> libc::c_int;

    // don't use directly
    fn My_IsNone(obj: PyObjectPointer) -> libc::c_int;
    // don't use directly
    fn My_IsTrue(obj: PyObjectPointer) -> libc::c_int;
    // don't use directly
    fn My_IsFalse(obj: PyObjectPointer) -> libc::c_int;
    fn My_Is(obj0: PyObjectPointer, obj1: PyObjectPointer) -> libc::c_int;
}

predicate! {
    fn all_other_unchanged(changed: PyObjectPointer, s0: &PyGlobalState, s: &PyGlobalState) -> bool {
        forall(|q: PyObjectPointer| (q.is_valid_pyobject(s0) && q !== changed ==> q.is_valid_pyobject(s) && q.get_data(s0) === q.get_data(s) && q.get_refcount(s0) == q.get_refcount(s)))
    }
}

predicate! {
    fn all_old_ones_unchanged_and_distinct_from_this(newobj: PyObjectPointer, s0: &PyGlobalState, s: &PyGlobalState) -> bool {
        forall(|q: PyObjectPointer| (q.is_valid_pyobject(s0) ==> q !== newobj && q.is_valid_pyobject(s) && q.get_data(s0) === q.get_data(s) && q.get_refcount(s0) == q.get_refcount(s)))
    }
}

#[trusted]
#[requires(obj.is_valid_pyobject(s))]
#[ensures(obj.is_valid_pyobject(s))]
#[ensures(obj.get_data(s) === old(obj.get_data(s)))]
#[ensures(all_other_unchanged(obj, old(s), s))]
#[ensures(obj.is_valid_pyobject(s) && obj.get_refcount(s) == old(obj.get_refcount(s)) + 1 && obj.get_data(s) === old(obj.get_data(s)))]
unsafe fn pyt_incref(obj: PyObjectPointer, s: &mut PyGlobalState) {
    Py_IncRef(obj)
}

#[trusted]
#[requires(obj.is_valid_pyobject(s))]
#[requires(!obj.is_held_by_api() || obj.get_refcount(s) >= 2)]
#[ensures(old(obj.get_refcount(s)) > 1 ==> obj.is_valid_pyobject(s) && obj.get_refcount(s) == old(obj.get_refcount(s)) - 1 && obj.get_data(s) === old(obj.get_data(s)))]
#[ensures(all_other_unchanged(obj, old(s), s))]
unsafe fn pyt_decref(obj: PyObjectPointer, s: &mut PyGlobalState) {
    Py_DecRef(obj)
}

#[trusted]
#[pure]
#[requires(obj.is_valid_pyobject(s))]
#[ensures(result == obj.get_refcount(s))]
unsafe fn pyt_refcnt(obj: PyObjectPointer, s: &PyGlobalState) -> Py_ssize_t {
    Py_REFCNT(obj)
}

#[trusted]
#[ensures(!result.is_null() ==> result.is_valid_pyobject(s) && (matches!(result.get_data(s), PyObjectData::PyLong)) && !result.is_held_by_api())]
#[ensures(all_other_unchanged(result, old(s), s))]
#[ensures(!result.is_null() && result.is_valid_pyobject(old(s)) ==> result.get_refcount(s) == result.get_refcount(old(s)) + 1 && result.get_data(old(s)) === result.get_data(s))]
#[ensures(!result.is_null() && !result.is_valid_pyobject(old(s)) ==> result.get_refcount(s) == 1)]
unsafe fn pytlong_fromlong(v: libc::c_long, s: &mut PyGlobalState) -> PyObjectPointer {
    PyLong_FromLong(v)
}

// TODO: take into account that this can raise an OverflowError
/*
#[trusted]
#[pure]
#[requires(obj.is_valid_pyobject(s) && matches!(obj.get_data(s), PyObjectData::PyLong))]
unsafe fn pytlong_aslong(obj: PyObjectPointer, s: &PyGlobalState) -> libc::c_long {
    PyLong_AsLong(obj)
}
*/

#[trusted]
#[pure]
#[requires(obj.is_valid_pyobject(s))]
#[ensures(result == matches!(obj.get_data(s), PyObjectData::PyLong))]
unsafe fn pylong_checkexact(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    MyLong_CheckExact(obj) != 0
}

#[trusted]
#[ensures(!result.is_null() ==> result.is_valid_pyobject(s) && matches!(result.get_data(s), PyObjectData::PyDict) && result.get_refcount(s) == 1 && !result.is_held_by_api())]
#[ensures(all_old_ones_unchanged_and_distinct_from_this(result, old(s), s))]
unsafe fn pytdict_new(s: &mut PyGlobalState) -> PyObjectPointer {
    PyDict_New()
}

#[trusted]
#[pure]
#[requires(obj.is_valid_pyobject(s))]
#[ensures(result == matches!(obj.get_data(s), PyObjectData::PyDict))]
unsafe fn pytdict_checkexact(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    MyDict_CheckExact(obj) != 0
}

#[trusted]
#[pure]
#[ensures(result.is_valid_pyobject(s) && result.is_held_by_api())]
#[ensures(matches!(result.get_data(s), PyObjectData::PyNone))]
unsafe fn pytnone_getnoincref(s: &PyGlobalState) -> PyObjectPointer {
    MyNone_GetNoIncRef()
}

#[trusted]
#[pure]
#[ensures(result.is_valid_pyobject(s) && result.is_held_by_api())]
#[ensures(matches!(result.get_data(s), PyObjectData::PyBool))]
unsafe fn pytbool_gettruenoincref(s: &PyGlobalState) -> PyObjectPointer {
    MyBool_GetTrueNoIncRef()
}

#[trusted]
#[pure]
#[ensures(result.is_valid_pyobject(s) && result.is_held_by_api())]
#[ensures(matches!(result.get_data(s), PyObjectData::PyBool))]
#[ensures(result !== unsafe { pytbool_gettruenoincref(&s) })]
unsafe fn pytbool_getfalsenoincref(s: &PyGlobalState) -> PyObjectPointer {
    MyBool_GetFalseNoIncRef()
}

#[trusted]
#[pure]
#[requires(obj.is_valid_pyobject(s))]
#[ensures(result <==> matches!(obj.get_data(s), PyObjectData::PyBool))]
#[ensures(result <==> unsafe { (pyt_istrue(obj, s) || pyt_isfalse(obj, s)) })]
unsafe fn pytbool_check(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    MyBool_Check(obj) != 0
}

#[trusted]
#[pure]
#[ensures(result <==> (obj.is_valid_pyobject(s) && matches!(obj.get_data(s), PyObjectData::PyNone)))]
#[ensures(result <==> obj === unsafe { pytnone_getnoincref(s) })]
unsafe fn pyt_isnone(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    My_IsNone(obj) != 0
}

#[trusted]
#[pure]
#[ensures(result <==> obj === unsafe { pytbool_gettruenoincref(s) })]
unsafe fn pyt_istrue(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    My_IsTrue(obj) != 0
}

#[trusted]
#[pure]
#[ensures(result <==> obj === unsafe { pytbool_gettruenoincref(s) })]
unsafe fn pyt_isfalse(obj: PyObjectPointer, s: &PyGlobalState) -> bool {
    My_IsFalse(obj) != 0
}

#[extern_spec(crate)]
extern "C" {
    #[pure]
    #[ensures(result != 0 <==> obj0 === obj1)]
    fn My_Is(obj0: PyObjectPointer, obj1: PyObjectPointer) -> libc::c_int;
}

#[trusted]
fn pri(v: libc::c_long) {
    println!("{}", v);
}

fn main() {
    unsafe {
        Py_Initialize();
    }

    let mut s = PyGlobalState { _private: 31415926 };

    unsafe {
        let lo = pytlong_fromlong(33, &mut s);
        if !lo.is_null() {
            let initial_rc = lo.get_refcount(&s);
            pyt_incref(lo, &mut s);
            pyt_incref(lo, &mut s);
            pyt_decref(lo, &mut s);
            pyt_decref(lo, &mut s);
            prusti_assert!(lo.get_refcount(&s) == initial_rc);
            //pri(pytlong_aslong(lo, &s));
        }
    }

    unsafe {
        let lo = pytlong_fromlong(43, &mut s);
        prusti_assume!(!lo.is_null());
        let initial_rc = lo.get_refcount(&s);
        let olo = lo;
        pyt_incref(olo, &mut s);
        prusti_assert!(lo.get_refcount(&s) == initial_rc + 1);
    }

    unsafe {
        let a = pytlong_fromlong(12, &mut s);
        let b = pytlong_fromlong(43, &mut s);

        prusti_assume!(!a.is_null());
        prusti_assume!(!b.is_null());

        assert!(pylong_checkexact(a, &s));
        assert!(pylong_checkexact(b, &s));
    }

    unsafe {
        let dict0 = pytdict_new(&mut s);
        let dict1 = pytdict_new(&mut s);
        let num = pytlong_fromlong(512, &mut s);

        prusti_assume!(!dict0.is_null());
        prusti_assume!(!dict1.is_null());
        prusti_assume!(!num.is_null());

        prusti_assert!(pytdict_checkexact(dict0, &s));
        prusti_assert!(!pylong_checkexact(dict0, &s));
        prusti_assert!(pytdict_checkexact(dict1, &s));
        prusti_assert!(pylong_checkexact(num, &s));

        prusti_assert!(dict0 !== dict1);
        prusti_assert!(dict0 !== num);
        prusti_assert!(dict1 !== num);
    }

    unsafe {
        let py_false = pytbool_getfalsenoincref(&s);
        let py_true = pytbool_gettruenoincref(&s);

        prusti_assert!(pytbool_check(py_false, &s));
        prusti_assert!(pytbool_check(py_true, &s));

        prusti_assert!(!pylong_checkexact(py_true, &s));

        prusti_assert!(py_false !== py_true);

        prusti_assert!(pyt_istrue(py_true, &s));
        prusti_assert!(pyt_isfalse(py_false, &s));
        prusti_assert!(!pyt_istrue(py_false, &s));
        prusti_assert!(!pyt_isfalse(py_true, &s));
    }

    unsafe {
        let py_none = pytnone_getnoincref(&s);
        let a_five = pytlong_fromlong(5, &mut s);

        prusti_assert!(pyt_isnone(py_none, &s));
        prusti_assert!(!pyt_isnone(a_five, &s));

        let another_py_none = pytnone_getnoincref(&s);
        prusti_assert!(py_none === another_py_none);
    }
}
