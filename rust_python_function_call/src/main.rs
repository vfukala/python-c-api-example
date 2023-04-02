use std::ffi::CString;
use std::io::{self, Write};

#[repr(C)]
struct PyObject {
    _private: [u8; 0]
}

type FfiString = *const libc::c_char;

static Py_eval_input: libc::c_int = 258;

extern "C" {
    fn Py_Initialize();
    fn Py_DecRef(obj: *mut PyObject);
    fn Py_IsNone(obj: *mut PyObject) -> libc::c_int;

    fn PyErr_Occurred() -> *mut PyObject;

    fn PyRun_SimpleString(s: FfiString) -> libc::c_int;
    fn PyRun_String(s: FfiString, mode: libc::c_int, globals: *const PyObject, locals: *const PyObject);

    fn PyImport_ImportModule(s: FfiString) -> *mut PyObject;

    fn PyObject_CallNoArgs(f: *const PyObject) -> *mut PyObject;
    fn PyObject_CallOneArg(f: *const PyObject, arg: *const PyObject) -> *mut PyObject;
    fn PyObject_GetAttrString(obj: *const PyObject, astr: FfiString) -> *mut PyObject;

    fn PyLong_AsLong(obj: *const PyObject) -> libc::c_long;
    fn PyLong_FromLong(v: libc::c_long) -> *mut PyObject;
    fn MyLong_Check(obj: *const PyObject) -> libc::c_int;

    fn PyDict_New() -> *mut PyObject;
    fn PyDict_SetItemString(dp: *mut PyObject, key: FfiString, item: *const PyObject) -> libc::c_int;
}

fn python_initialize() {
    unsafe {
        Py_Initialize();
    }
}

fn python_decref(obj: *mut PyObject) {
    unsafe {
        Py_DecRef(obj)
    }
}

fn python_is_none(obj: *mut PyObject) -> bool {
    unsafe {
        Py_IsNone(obj) != 0
    }
}

fn python_err_occurred() -> *mut PyObject {
    unsafe {
        PyErr_Occurred()
    }
}

fn to_c_string(s: &str) -> CString {
    CString::new(s).expect("")
}

fn python_run_simple_string(s: &str) {
    unsafe {
        let c_str = to_c_string(s);
        PyRun_SimpleString(c_str.as_ptr());
    }
}

fn python_run_string(s: &str, globals: *const PyObject, locals: *const PyObject) {
    unsafe {
        let c_str = to_c_string(s);
        PyRun_String(c_str.as_ptr(), Py_eval_input, globals, locals);
    }
}

fn python_import_module(s: &str) -> *mut PyObject {
    unsafe {
        let c_str = to_c_string(s);
        PyImport_ImportModule(c_str.as_ptr())
    }
}

fn python_call_no_args(f: *const PyObject) -> *mut PyObject {
    unsafe {
        PyObject_CallNoArgs(f)
    }
}

fn python_call_one_arg(f: *const PyObject, arg: *const PyObject) -> *mut PyObject {
    unsafe {
        PyObject_CallOneArg(f, arg)
    }
}

fn python_get_attribute(obj: *const PyObject, astr: &str) -> *mut PyObject {
    unsafe {
        let a_c_str = to_c_string(astr);
        PyObject_GetAttrString(obj, a_c_str.as_ptr())
    }
}

fn python_from_long(v: i64) -> *mut PyObject {
    unsafe {
        PyLong_FromLong(v)
    }
}

fn python_long_check(obj: *const PyObject) -> bool {
    unsafe {
        MyLong_Check(obj) != 0
    }
}

fn python_as_long(obj: *const PyObject) -> i64 {
    unsafe {
        PyLong_AsLong(obj)
    }
}

fn python_dict_new() -> *mut PyObject {
    unsafe {
        PyDict_New()
    }
}

fn python_dict_set_item_string(dict: *mut PyObject, key: &str, item: *const PyObject) {
    unsafe {
        let c_str = to_c_string(key);
        PyDict_SetItemString(dict, c_str.as_ptr(), item);
    }
}

fn print_pyobject(obj: *const PyObject) {
    if obj.is_null() {
        println!("(((can't print NULL as a PyObject)))");
    } else {
        let globals = python_dict_new();
        python_dict_set_item_string(globals, "x", obj);
        let locals = python_dict_new();
        python_run_string("print(x)", globals, locals);
        python_decref(globals);
        python_decref(locals);
    }
}

fn main() {
    python_initialize();

    python_run_simple_string("import sys\nsys.path.append('')");

    let lib_module = python_import_module("lib");

    let give_five = python_get_attribute(lib_module, "give_five");
    let give_two = python_get_attribute(lib_module, "give_two");
    let take_five = python_get_attribute(lib_module, "take_five");
    let give_list_a = python_get_attribute(lib_module, "give_list_a");
    let get_sorted_list = python_get_attribute(lib_module, "get_sorted_list");
    let get_cubed = python_get_attribute(lib_module, "get_cubed");
    let get_binary = python_get_attribute(lib_module, "get_binary");

    let five = python_call_no_args(give_five);
    let two = python_call_no_args(give_two);
    let list_a = python_call_no_args(give_list_a);

    let sorted_list = python_call_one_arg(get_sorted_list, list_a);

    print!("list_a: ");
    io::stdout().flush().unwrap();
    print_pyobject(list_a);

    print!("sorted_list: ");
    io::stdout().flush().unwrap();
    print_pyobject(sorted_list);

    let seven_cubed = python_call_one_arg(get_cubed, python_from_long(7));
    assert!(python_long_check(seven_cubed));

    println!("seved cubed is {}", python_as_long(seven_cubed));
    let some_binary = python_call_one_arg(get_binary, python_from_long(257));

    print!("binary of 257: ");
    io::stdout().flush().unwrap();
    print_pyobject(some_binary);

    let take_five_ret_five = python_call_one_arg(take_five, five);
    assert!(python_err_occurred().is_null());
    assert!(python_is_none(take_five_ret_five));

    /*let take_five_ret_two = */python_call_one_arg(take_five, two);
    assert!(!python_err_occurred().is_null());
}
