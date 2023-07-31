use prusti_contracts::*;

pub type pyt_ssize_t = isize;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PytObjectPointer {
    pub _private: usize
}

type GpyErrorState = bool;

pub struct GpyGlobalState {
    _private: usize, // this represents opaque data (the heap of PyObjects)
    error: GpyErrorState,
    constants: GpyConstantObjects,
}

struct GpyConstantObjects {
    py_none: PytObjectPointer,
    py_false: PytObjectPointer,
    py_true: PytObjectPointer,
    py_not_implemented: PytObjectPointer,
}

predicate! {
    fn has_value_and_pos_ref_count(obj: PytObjectPointer, data: GpyObjectData, s: &GpyGlobalState) -> bool {
        (obj.gpy_get(s).data === data) & (obj.gpy_get(s).ref_count >= 1)
    }
}

#[trusted]
#[ensures(!result.error)]
#[ensures(has_value_and_pos_ref_count(result.constants.py_none, GpyObjectData::PyNone, &result))]
#[ensures(has_value_and_pos_ref_count(result.constants.py_false, GpyObjectData::PyBool(false), &result))]
#[ensures(has_value_and_pos_ref_count(result.constants.py_none, GpyObjectData::PyBool(true), &result))]
#[ensures(has_value_and_pos_ref_count(result.constants.py_none, GpyObjectData::PyNotImplemented, &result))]
pub fn gpy_create_state() -> GpyGlobalState {
    GpyGlobalState {
        _private: 0,
        error: false,
        constants: GpyConstantObjects {
            py_none: PytObjectPointer { _private: 0 },
            py_false: PytObjectPointer { _private: 0 },
            py_true: PytObjectPointer { _private: 0 },
            py_not_implemented: PytObjectPointer { _private: 0 },
        }
    }
}

obligation! {
    pub fn gpy_ref_held(amount: usize, obj: PytObjectPointer);
}

obligation! {
    fn gpy_initialized(amount: usize);
}

impl PytObjectPointer {
    #[pure]
    pub fn is_null(&self) -> bool {
        self._private == 0
    }

    // spec-only
    #[trusted]
    #[pure]
    pub fn gpy_get(&self, s: &GpyGlobalState) -> GpyObject {
        unreachable!()
    }
}

#[derive(Clone, Copy)]
struct GpyObject {
    ref_count: pyt_ssize_t,
    data: GpyObjectData,
    typ: PytObjectPointer,
}

#[derive(Clone, Copy)]
enum GpyObjectData {
    PyNone,
    PyLong,
    PyBool(bool),
    PyDict,
    PyNotImplemented,
    //PyType(GpyType),
}

impl GpyObjectData {
    #[pure]
    fn is_bool(&self) -> bool {
        matches!(self, GpyObjectData::PyBool(_))
    }
}

trait GpyType {
    #[pure]
    fn getattr(obj: PytObjectPointer, name: PytObjectPointer, s: &GpyGlobalState) -> (PytObjectPointer, GpyErrorState);
}

struct GpyLongType {}


/*
#[derive(Clone)]
struct GpyType {
    getattr: Option<fn(obj: PytObjectPointer, name: PytObjectPointer, s: &GpyGlobalState) -> (PytObjectPointer, GpyErrorState)>,
    //setattr: Option<dyn GpySetattr>,
}
*/

/*
trait GpyGetattr {
    // returns the result and the updated error state
    #[pure]
    fn getattr(obj: PytObjectPointer, name: PytObjectPointer, s: &GpyGlobalState) -> (PytObjectPointer, GpyErrorState);
}

trait GpySetattr {
    // returns the updated object and the updated error state
    #[pure]
    fn setattr(obj: PytObjectPointer, name: PytObjectPointer, s: &GpyGlobalState) -> (GpyObjectData, GpyErrorState);
}
*/

extern "C" {
    fn Py_Initialize();
    fn Py_Finalize();
    fn Py_IncRef(obj: PytObjectPointer);
    fn Py_DecRef(obj: PytObjectPointer);
    fn Py_REFCNT(obj: PytObjectPointer) -> pyt_ssize_t;

    fn PyLong_AsLong(obj: PytObjectPointer) -> libc::c_long;
    fn PyLong_FromLong(v: libc::c_long) -> PytObjectPointer;
    fn MyLong_CheckExact(obj: PytObjectPointer) -> libc::c_int;

    fn PyDict_New() -> PytObjectPointer;
    fn MyDict_CheckExact(obj: PytObjectPointer) -> libc::c_int;

    fn MyNone_GetNoIncRef() -> PytObjectPointer;
    fn MyBool_GetTrueNoIncRef() -> PytObjectPointer;
    fn MyBool_GetFalseNoIncRef() -> PytObjectPointer;

    fn MyBool_Check(obj: PytObjectPointer) -> libc::c_int;

    fn My_IsNone(obj: PytObjectPointer) -> libc::c_int;
    fn My_IsTrue(obj: PytObjectPointer) -> libc::c_int;
    fn My_IsFalse(obj: PytObjectPointer) -> libc::c_int;
    fn My_Is(obj0: PytObjectPointer, obj1: PytObjectPointer) -> libc::c_int;

    fn PyObject_HasAttr(obj: PytObjectPointer, name: PytObjectPointer) -> libc::c_int;
}

predicate! {
    fn all_other_preserved(changed: PytObjectPointer, s0: &GpyGlobalState, s: &GpyGlobalState) -> bool {
        forall(|q: PytObjectPointer| (q !== changed ==> q.gpy_get(s) === q.gpy_get(s0)))
    }
}

predicate! {
    fn all_objects_preserved(s0: &GpyGlobalState, s: &GpyGlobalState) -> bool {
        forall(|q: PytObjectPointer| q.gpy_get(s) === q.gpy_get(s0))
    }
}

predicate! {
    fn errors_preserved(s0: &GpyGlobalState, s: &GpyGlobalState) -> bool {
        s0.error == s.error
    }
}

predicate! {
    fn constants_preserved(s0: &GpyGlobalState, s: &GpyGlobalState) -> bool {
        s0.constants === s.constants
    }
}

#[trusted]
#[ensures(gpy_initialized(1))]
pub unsafe fn pyt_initialize() {
    Py_Initialize();
}

#[trusted]
#[requires(gpy_initialized(1))]
pub unsafe fn pyt_finalize() {
    Py_Finalize();
}

#[trusted]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(gpy_initialized(1))]
#[ensures(gpy_ref_held(2, obj))]
#[ensures(obj.gpy_get(old(s)).ref_count + 1 == obj.gpy_get(s).ref_count)]
#[ensures(all_other_preserved(obj, old(s), s))]
#[ensures(errors_preserved(old(s), s))]
#[ensures(constants_preserved(old(s), s))]
pub unsafe fn pyt_incref(obj: PytObjectPointer, s: &mut GpyGlobalState) {
    Py_IncRef(obj)
}

#[trusted]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(gpy_initialized(1))]
#[ensures(obj.gpy_get(old(s)).ref_count - 1 == obj.gpy_get(s).ref_count)]
#[ensures(all_other_preserved(obj, old(s), s))]
#[ensures(errors_preserved(old(s), s))]
#[ensures(constants_preserved(old(s), s))]
pub unsafe fn pyt_decref(obj: PytObjectPointer, s: &mut GpyGlobalState) {
    Py_DecRef(obj)
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(result == obj.gpy_get(s).ref_count)]
pub unsafe fn pyt_refcnt(obj: PytObjectPointer, s: &GpyGlobalState) -> pyt_ssize_t {
    Py_REFCNT(obj)
}

#[trusted]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(gpy_initialized(1))]
#[ensures(gpy_ref_held(1, obj))]
#[ensures(all_objects_preserved(old(s), s))]
#[ensures(constants_preserved(old(s), s))]
pub unsafe fn pytlong_aslong(obj: PytObjectPointer, s: &mut GpyGlobalState) -> libc::c_long {
    PyLong_AsLong(obj)
}

#[trusted]
#[requires(gpy_initialized(1))]
#[ensures(gpy_initialized(1))]
#[ensures(!result.is_null() ==> gpy_ref_held(1, result) & all_other_preserved(result, old(s), s) & (result.gpy_get(s).ref_count == result.gpy_get(old(s)).ref_count + 1) & (result.gpy_get(s).data === GpyObjectData::PyLong))]
#[ensures(result.is_null() ==> all_objects_preserved(old(s), s))]
#[ensures(result.is_null() ==> s.error)]
#[ensures(!result.is_null() ==> errors_preserved(old(s), s))]
#[ensures(constants_preserved(old(s), s))]
pub unsafe fn pytlong_fromlong(v: libc::c_long, s: &mut GpyGlobalState) -> PytObjectPointer {
    PyLong_FromLong(v)
}


#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(result <==> obj.gpy_get(s).data === GpyObjectData::PyLong)]
pub unsafe fn pylong_checkexact(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    MyLong_CheckExact(obj) != 0
}

#[trusted]
#[requires(gpy_initialized(1))]
#[ensures(gpy_initialized(1))]
#[ensures(!result.is_null() ==> gpy_ref_held(1, result) & (result.gpy_get(old(s)).ref_count == 0) & (result.gpy_get(s).data === GpyObjectData::PyDict) & (result.gpy_get(s).ref_count == 1))]
#[ensures(result.is_null() ==> all_objects_preserved(old(s), s))]
#[ensures(result.is_null() ==> s.error)]
#[ensures(!result.is_null() ==> errors_preserved(old(s), s))]
#[ensures(constants_preserved(old(s), s))]
pub unsafe fn pytdict_new(s: &mut GpyGlobalState) -> PytObjectPointer {
    PyDict_New()
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(result <==> obj.gpy_get(s).data === GpyObjectData::PyDict)]
pub unsafe fn pytdict_checkexact(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    MyDict_CheckExact(obj) != 0
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result === s.constants.py_none)]
pub unsafe fn pytnone_getnoincref(s: &GpyGlobalState) -> PytObjectPointer {
    MyNone_GetNoIncRef()
}

#[trusted]
#[requires(gpy_initialized(1))]
#[ensures(gpy_initialized(1))]
#[ensures(gpy_ref_held(1, result))]
#[ensures(result === s.constants.py_none)]
#[ensures(constants_preserved(old(s), s))]
#[ensures(all_other_preserved(s.constants.py_none, old(s), s))]
#[ensures(s.constants.py_none.gpy_get(s).data === s.constants.py_none.gpy_get(old(s)).data)]
#[ensures(s.constants.py_none.gpy_get(s).ref_count == s.constants.py_none.gpy_get(old(s)).ref_count + 1)]
pub unsafe fn pytnone_get(s: &mut GpyGlobalState) -> PytObjectPointer {
    let obj = MyNone_GetNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result === s.constants.py_true)]
pub unsafe fn pytbool_gettruenoincref(s: &GpyGlobalState) -> PytObjectPointer {
    MyBool_GetTrueNoIncRef()
}

#[trusted]
#[requires(gpy_initialized(1))]
#[ensures(gpy_initialized(1))]
#[ensures(gpy_ref_held(1, result))]
#[ensures(result === s.constants.py_true)]
#[ensures(constants_preserved(old(s), s))]
#[ensures(all_other_preserved(s.constants.py_true, old(s), s))]
#[ensures(s.constants.py_true.gpy_get(s).data === s.constants.py_true.gpy_get(old(s)).data)]
#[ensures(s.constants.py_true.gpy_get(s).ref_count == s.constants.py_true.gpy_get(old(s)).ref_count + 1)]
pub unsafe fn pytbool_gettrue(s: &GpyGlobalState) -> PytObjectPointer {
    let obj = MyBool_GetTrueNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result === s.constants.py_false)]
pub unsafe fn pytbool_getfalsenoincref(s: &GpyGlobalState) -> PytObjectPointer {
    MyBool_GetFalseNoIncRef()
}

#[trusted]
#[requires(gpy_initialized(1))]
#[ensures(gpy_initialized(1))]
#[ensures(gpy_ref_held(1, result))]
#[ensures(result === s.constants.py_false)]
#[ensures(constants_preserved(old(s), s))]
#[ensures(all_other_preserved(s.constants.py_false, old(s), s))]
#[ensures(s.constants.py_false.gpy_get(s).data === s.constants.py_false.gpy_get(old(s)).data)]
#[ensures(s.constants.py_false.gpy_get(s).ref_count == s.constants.py_false.gpy_get(old(s)).ref_count + 1)]
pub unsafe fn pytbool_getfalse(s: &GpyGlobalState) -> PytObjectPointer {
    let obj = MyBool_GetFalseNoIncRef();
    Py_IncRef(obj);
    obj
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[requires(gpy_ref_held(1, obj))]
#[ensures(result == obj.gpy_get(s).data.is_bool())]
pub unsafe fn pytbool_check(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    MyBool_Check(obj) != 0
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result <==> obj == s.constants.py_none)]
pub unsafe fn pyt_isnone(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    My_IsNone(obj) != 0
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result <==> obj == s.constants.py_true)]
pub unsafe fn pyt_istrue(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    My_IsTrue(obj) != 0
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result <==> obj == s.constants.py_false)]
pub unsafe fn pyt_isfalse(obj: PytObjectPointer, s: &GpyGlobalState) -> bool {
    My_IsFalse(obj) != 0
}

#[trusted]
#[pure]
#[requires(gpy_initialized(1))]
#[ensures(result <==> obj0 == obj1)]
pub unsafe fn pyt_is(obj0: PytObjectPointer, obj1: PytObjectPointer) -> bool {
    My_Is(obj0, obj1) != 0
}

/*
pub unsafe fn pytobject_has_attr(obj0: PytObjectPointer, obj1: PytObjectPointer) -> bool {

}
*/
