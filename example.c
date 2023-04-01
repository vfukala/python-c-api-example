#include <python3.10/Python.h>

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

bool typecheck_binary_search(PyObject *list, PyObject *target)
{
	if (!list)
	{
		puts("the first argument is NULL!");
		return false;
	}

	if (!PyList_Check(list))
	{
		puts("the first argument is not a list!");
		return false;
	}

	if (!target)
	{
		puts("the second argument is NULL!");
		return false;
	}

	if (!PyLong_Check(target))
	{
		puts("the second argument is not a long object (it's not an integer)!");
		return false;
	}

	const Py_ssize_t len = PyList_Size(list);
	for (Py_ssize_t i = 0; i < len; i++)
	{
		PyObject *item = PyList_GetItem(list, i);
		if (!item)
		{
			printf("the item at position %ld in the list is NULL!\n", i);
			return false;
		}
		if (!PyLong_Check(item))
		{
			printf("the item at position %ld in the list is not a long object (it's not an integer)!\n", i);
			return false;
		}
	}

	return true;
}

bool binary_search_c_like(PyObject *list, PyObject *target)
{
	if (!typecheck_binary_search(list, target))
		return false;

	const long c_target = PyLong_AsLong(target);
	size_t low = 0, high = PyList_Size(list);
	while (low < high)
	{
		const size_t mid = (low + high) / 2;
		PyObject *mid_item = PyList_GetItem(list, mid);
		const long mid_val = PyLong_AsLong(mid_item);
		if (mid_val < c_target)
			low = mid + 1;
		else if (mid_val > c_target)
			high = mid;
		else
			return true;
	}

	return false;
}


// use b == NULL for unary operations
PyObject *binary_op_py_longs(const char *a_b_string, PyObject *a, PyObject *b, PyTypeObject *expected_type)
{
	PyObject *vars = PyDict_New();
	PyDict_SetItemString(vars, "a", a);

	if (b)
		PyDict_SetItemString(vars, "b", b);

	PyObject *empty_dict = PyDict_New();

	PyObject *result = PyRun_String(a_b_string, Py_eval_input, vars, empty_dict);
	assert(Py_IS_TYPE(result, expected_type));

	Py_DECREF(vars);
	Py_DECREF(empty_dict);

	return result;
}

PyObject *average_py_longs(PyObject *a, PyObject *b)
{
	return binary_op_py_longs("(a + b) // 2", a, b, &PyLong_Type);
}

PyObject *plus_one_py_long(PyObject *a)
{
	return binary_op_py_longs("a + 1", a, NULL, &PyLong_Type);
}

PyObject *binary_search_python_like(PyObject *list, PyObject *target)
{
	if (!typecheck_binary_search(list, target))
		return NULL;

	PyObject *low = PyLong_FromLong(0), *high = PyLong_FromLong(PyList_Size(list));
	while (true)
	{
		// loop condition
		{
			PyObject *comparison_result = PyObject_RichCompare(low, high, Py_LT);
			assert(PyBool_Check(comparison_result));
			const bool was_false = !PyObject_IsTrue(comparison_result);
			Py_DECREF(comparison_result);
			if (was_false)
				break;
		}

		// loop body
		PyObject *mid = average_py_longs(low, high);
		const long mid_long = PyLong_AsLong(mid);
		PyObject *mid_item = PyList_GetItem(list, mid_long);
		PyObject *less_cmp_result = PyObject_RichCompare(mid_item, target, Py_LT);
		assert(PyBool_Check(less_cmp_result));
		PyObject *greater_cmp_result = PyObject_RichCompare(mid_item, target, Py_GT);
		assert(PyBool_Check(greater_cmp_result));
		bool found = false;
		if (PyObject_IsTrue(less_cmp_result))
		{
			Py_DECREF(low);
			low = plus_one_py_long(mid);
		}
		else if (PyObject_IsTrue(greater_cmp_result))
		{
			Py_DECREF(high);
			high = mid;
			Py_INCREF(high);
		}
		else
		{
			found = true;
		}

		Py_DECREF(mid);
		Py_DECREF(less_cmp_result);
		Py_DECREF(greater_cmp_result);

		if (found)
		{
			Py_DECREF(low);
			Py_DECREF(high);

			Py_INCREF(Py_True);
			return Py_True;
		}
	}
	Py_INCREF(Py_False);
	return Py_False;
}

void print_pyobject(PyObject* obj)
{
	if (!obj)
	{
		puts("(((can't print NULL as a PyObject)))");
		return;
	}
	PyObject *globals = PyDict_New();
	PyDict_SetItemString(globals, "x", obj);
	PyRun_String("print(x)", Py_eval_input, globals, PyDict_New());
}

void test_binary_search(PyObject *list, PyObject *target)
{
	puts("BINARY SEARCH on");
	print_pyobject(list);
	puts("with target");
	print_pyobject(target);
	putchar('\n');

	puts(">>>> C version");
	const bool c_result = binary_search_c_like(list, target);
	printf("C version result:\n%d\n\n", c_result);

	puts(">>>> Python version");
	PyObject *python_result = binary_search_python_like(list, target);
	puts("python version result:");
	print_pyobject(python_result);
	puts("\n\n");
}

void test1()
{
	PyObject *list = PyList_New(3);

	PyObject *target = PyLong_FromLong(1);

	test_binary_search(list, target);

	Py_DECREF(list);
	Py_DECREF(target);
}

void test2()
{
	PyObject *list = PyList_New(2);
	PyList_SetItem(list, 0, PyLong_FromLong(23));
	PyList_SetItem(list, 1, PyUnicode_FromString("stray cat"));

	PyObject *target = PyLong_FromLong(23);

	test_binary_search(list, target);

	Py_DECREF(list);
	Py_DECREF(target);
}

void test3()
{
	const Py_ssize_t n = 17;

	PyObject *list = PyList_New(n);

	for (Py_ssize_t i = 0; i < n; i++)
		PyList_SetItem(list, i, PyLong_FromLong(i * i));

	PyObject *target1 = PyLong_FromLong(24);
	PyObject *target2 = PyLong_FromLong(25);

	test_binary_search(list, target1);
	test_binary_search(list, target2);

	Py_DECREF(list);
	Py_DECREF(target1);
	Py_DECREF(target2);
}

int main(void)
{
	Py_Initialize();

	test1();
	test2();
	test3();

	return EXIT_SUCCESS;
}
