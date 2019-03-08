use crate::ffi3::object::*;
use std::os::raw::c_int;

pub enum PyWeakReference {}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    static mut _PyWeakref_RefType: PyTypeObject;
    static mut _PyWeakref_ProxyType: PyTypeObject;
    static mut _PyWeakref_CallableProxyType: PyTypeObject;
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyWeakref_CheckRef")]
pub unsafe fn PyWeakref_CheckRef(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut _PyWeakref_RefType)
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyWeakref_CheckRefExact")]
pub unsafe fn PyWeakref_CheckRefExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut _PyWeakref_RefType) as c_int
}

#[inline]
#[cfg_attr(PyPy, link_name = "PyPyWeakref_CheckProxy")]
pub unsafe fn PyWeakref_CheckProxy(op: *mut PyObject) -> c_int {
    ((Py_TYPE(op) == &mut _PyWeakref_ProxyType)
        || (Py_TYPE(op) == &mut _PyWeakref_CallableProxyType)) as c_int
}

#[inline]
pub unsafe fn PyWeakref_Check(op: *mut PyObject) -> c_int {
    (PyWeakref_CheckRef(op) != 0 || PyWeakref_CheckProxy(op) != 0) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyWeakref_NewRef")]
    pub fn PyWeakref_NewRef(ob: *mut PyObject, callback: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyWeakref_NewProxy")]
    pub fn PyWeakref_NewProxy(ob: *mut PyObject, callback: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyWeakref_GetObject")]
    pub fn PyWeakref_GetObject(_ref: *mut PyObject) -> *mut PyObject;
}
