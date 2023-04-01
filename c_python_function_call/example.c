#include <python3.12d/Python.h>

#include <stdio.h>
#include <stdlib.h>

void print_pyobject(PyObject* obj)
{
	if (!obj)
	{
		puts("(((can't print NULL as a PyObject)))");
		return;
	}
	PyObject *globals = PyDict_New();
	PyDict_SetItemString(globals, "x", obj);
	PyObject *locals = PyDict_New();
	PyRun_String("print(x)", Py_eval_input, globals, locals);
	Py_DECREF(globals);
	Py_DECREF(locals);
}

int main(void)
{
	Py_Initialize();

	// add pwd to path to be able to find the lib module for import
	PyRun_SimpleString("import sys\nsys.path.append('')");

	PyObject *lib_module = PyImport_ImportModule("lib");

	PyObject *give_five = PyObject_GetAttrString(lib_module, "give_five");
	PyObject *give_two = PyObject_GetAttrString(lib_module, "give_two");
	PyObject *take_five = PyObject_GetAttrString(lib_module, "take_five");
	PyObject *give_list_a = PyObject_GetAttrString(lib_module, "give_list_a");
	PyObject *get_sorted_list = PyObject_GetAttrString(lib_module, "get_sorted_list");
	PyObject *get_binary = PyObject_GetAttrString(lib_module, "get_binary");
	
	PyObject *five = PyObject_CallNoArgs(give_five);
	PyObject *two = PyObject_CallNoArgs(give_two);
	PyObject *list_a = PyObject_CallNoArgs(give_list_a);

	PyObject *sorted_list = PyObject_CallOneArg(get_sorted_list, list_a);

	fputs("list_a: ", stdout);
	fflush(stdout);
	print_pyobject(list_a);

	fputs("sorted_list: ", stdout);
	fflush(stdout);
	print_pyobject(sorted_list);

	PyObject *some_binary = PyObject_CallOneArg(get_binary, PyLong_FromLong(257));

	fputs("binary of 257: ", stdout);
	fflush(stdout);
	print_pyobject(some_binary);

	PyObject *take_five_ret_five = PyObject_CallOneArg(take_five, five);
	assert(!PyErr_Occurred());
	assert(Py_IsNone(take_five_ret_five));

	/* PyObject *take_five_ret_two = */PyObject_CallOneArg(take_five, two);
	assert(PyErr_Occurred());

	return EXIT_SUCCESS;
}
