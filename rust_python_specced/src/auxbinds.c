#include <python3.12d/Python.h>

int MyLong_CheckExact(PyObject *obj) {
	return PyLong_CheckExact(obj);
}

int MyDict_CheckExact(PyObject *obj) {
	return PyDict_CheckExact(obj);
}

PyObject *MyNone_GetNoIncRef() {
	return Py_None;
}

PyObject *MyBool_GetTrueNoIncRef() {
	return Py_True;
}

PyObject *MyBool_GetFalseNoIncRef() {
	return Py_False;
}

int MyBool_Check(PyObject *obj) {
	return PyBool_Check(obj);
}

int My_IsNone(PyObject *obj) {
	return Py_IsNone(obj);
}

int My_IsTrue(PyObject *obj) {
	return Py_IsTrue(obj);
}

int My_IsFalse(PyObject *obj) {
	return Py_IsFalse(obj);
}

int My_Is(PyObject *obj0, PyObject *obj1) {
	return Py_Is(obj0, obj1);
}
