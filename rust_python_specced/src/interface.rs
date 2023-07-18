use prusti_contracts::*;

pub type Py_ssize_t = isize;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyObjectPointer {
    _private: usize
}

obligation! {
    fn py_ref_held(amount: usize, obj: PyObjectPointer);
}

obligation! {
    fn py_initialized(amount: usize);
}

impl PyObjectPointer {
    #[pure]
    pub fn is_null(&self) -> bool {
        self._private == 0
    }
}

extern "C" {
    pub fn Py_Initialize();
    pub fn Py_Finalize();
    fn Py_IncRef(obj: PyObjectPointer);
    fn Py_DecRef(obj: PyObjectPointer);
    fn Py_REFCNT(obj: PyObjectPointer) -> Py_ssize_t;

    fn PyLong_AsLong(obj: PyObjectPointer) -> libc::c_long;
    fn PyLong_FromLong(v: libc::c_long) -> PyObjectPointer;
    fn MyLong_CheckExact(obj: PyObjectPointer) -> libc::c_int;

    fn PyDict_New() -> PyObjectPointer;
    fn MyDict_CheckExact(obj: PyObjectPointer) -> libc::c_int;

    fn MyNone_GetNoIncRef() -> PyObjectPointer;
    fn MyBool_GetTrueNoIncRef() -> PyObjectPointer;
    fn MyBool_GetFalseNoIncRef() -> PyObjectPointer;

    fn MyBool_Check(obj: PyObjectPointer) -> libc::c_int;

    fn My_IsNone(obj: PyObjectPointer) -> libc::c_int;
    fn My_IsTrue(obj: PyObjectPointer) -> libc::c_int;
    fn My_IsFalse(obj: PyObjectPointer) -> libc::c_int;
    pub fn My_Is(obj0: PyObjectPointer, obj1: PyObjectPointer) -> libc::c_int;
}

#[extern_spec(crate)]
extern "C" {
    #[ensures(py_initialized(1))]
    fn Py_Initialize();
    #[requires(py_initialized(1))]
    fn Py_Finalize();
}

#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
#[ensures(py_ref_held(2, obj))]
pub unsafe fn pyt_incref(obj: PyObjectPointer) {
    Py_IncRef(obj)
}

#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
pub unsafe fn pyt_decref(obj: PyObjectPointer) {
    Py_DecRef(obj)
}

#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
#[ensures(py_ref_held(1, obj))]
pub unsafe fn pyt_refcnt(obj: PyObjectPointer) -> Py_ssize_t {
    Py_REFCNT(obj)
}

#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
#[ensures(py_ref_held(1, obj))]
pub unsafe fn pytlong_aslong(obj: PyObjectPointer) -> libc::c_long {
    // TODO this might throw an error!!
    PyLong_AsLong(obj)
}

#[trusted]
#[requires(py_initialized(1))]
#[ensures(py_initialized(1))]
#[ensures(!result.is_null() ==> py_ref_held(1, result))]
pub unsafe fn pytlong_fromlong(v: libc::c_long) -> PyObjectPointer {
    PyLong_FromLong(v)
}


#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
#[ensures(py_ref_held(1, obj))]
pub unsafe fn pylong_checkexact(obj: PyObjectPointer) -> bool {
    MyLong_CheckExact(obj) != 0
}

#[trusted]
#[requires(py_initialized(1))]
#[ensures(py_initialized(1))]
#[ensures(!result.is_null() ==> py_ref_held(1, result))]
pub unsafe fn pytdict_new() -> PyObjectPointer {
    PyDict_New()
}

#[trusted]
#[requires(py_initialized(1))]
#[requires(py_ref_held(1, obj))]
#[ensures(py_initialized(1))]
#[ensures(py_ref_held(1, obj))]
pub unsafe fn pytdict_checkexact(obj: PyObjectPointer) -> bool {
    MyDict_CheckExact(obj) != 0
}

#[trusted]
#[pure]
// TODO require py_initialized
pub unsafe fn pytnone_getnoincref() -> PyObjectPointer {
    MyNone_GetNoIncRef()
}

#[trusted]
#[requires(py_initialized(1))]
#[ensures(py_initialized(1))]
#[ensures(result == unsafe { pytnone_getnoincref() })]
#[ensures(py_ref_held(1, result))]
pub unsafe fn pytnone_get() -> PyObjectPointer {
    let obj = MyNone_GetNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
// TODO require py_initialized
pub unsafe fn pytbool_gettruenoincref() -> PyObjectPointer {
    MyBool_GetTrueNoIncRef()
}

#[trusted]
#[requires(py_initialized(1))]
#[ensures(py_initialized(1))]
#[ensures(result == unsafe { pytbool_gettruenoincref() })]
#[ensures(py_ref_held(1, result))]
pub unsafe fn pytbool_gettrue() -> PyObjectPointer {
    let obj = MyBool_GetTrueNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
// TODO require py_initialized
pub unsafe fn pytbool_getfalsenoincref() -> PyObjectPointer {
    MyBool_GetFalseNoIncRef()
}

#[trusted]
#[requires(py_initialized(1))]
#[ensures(py_initialized(1))]
#[ensures(result == unsafe { pytbool_getfalsenoincref() })]
#[ensures(py_ref_held(1, result))]
pub unsafe fn pytbool_getfalse() -> PyObjectPointer {
    let obj = MyBool_GetFalseNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
// require ref held (?)
#[ensures(result <==> unsafe { (pyt_istrue(obj) || pyt_isfalse(obj)) })]
// TODO require py_initialized
pub unsafe fn pytbool_check(obj: PyObjectPointer) -> bool {
    MyBool_Check(obj) != 0
}

#[trusted]
// require ref held (?)
#[pure]
#[ensures(result <==> obj === unsafe { pytnone_getnoincref() })]
// TODO require py_initialized
pub unsafe fn pyt_isnone(obj: PyObjectPointer) -> bool {
    My_IsNone(obj) != 0
}

#[trusted]
// require ref held (?)
#[pure]
#[ensures(result <==> obj === unsafe { pytbool_gettruenoincref() })]
// TODO require py_initialized
pub unsafe fn pyt_istrue(obj: PyObjectPointer) -> bool {
    My_IsTrue(obj) != 0
}

#[trusted]
// require ref held (?)
#[pure]
#[ensures(result <==> obj === unsafe { pytbool_gettruenoincref() })]
// TODO require py_initialized
pub unsafe fn pyt_isfalse(obj: PyObjectPointer) -> bool {
    My_IsFalse(obj) != 0
}

#[extern_spec(crate)]
extern "C" {
    #[pure]
    #[ensures(result != 0 <==> obj0 === obj1)]
    fn My_Is(obj0: PyObjectPointer, obj1: PyObjectPointer) -> libc::c_int;
}
