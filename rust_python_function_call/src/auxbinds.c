#include <python3.12d/Python.h>

int MyLong_Check(PyObject *obj) {
	return PyLong_Check(obj);
}
