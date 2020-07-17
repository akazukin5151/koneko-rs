use pyo3::prelude::*;
use pyo3::types::PyDict;


pub struct Data<'a> {
    pub path: &'a str,
    pub size: i32,
    pub x: i32,
    pub y: i32,
}

pub fn run(py: Python, locals: &PyDict, data: Data) -> Result<(), ()> {
    let newlocals = set_locals(locals, data)
                    .unwrap_or_else(|_| panic!("PyO3 can not set a local variable!"));

    eval(py, &newlocals).map_err(|e| {
        // We can't display Python exceptions via std::fmt::Display,
        // so print the error here manually.
        e.print_and_set_sys_last_vars(py);
    })
}

fn set_locals<'a>(locals: &'a PyDict, data: Data) -> PyResult<&'a PyDict> {
    locals.set_item("path", data.path)?;
    locals.set_item("size", data.size)?;
    locals.set_item("x", data.x)?;
    locals.set_item("y", data.y)?;
    Ok(locals)
}

fn eval(py: Python, locals: &PyDict) -> PyResult<()> {
    let code = "pixcat.Image(path).thumbnail(size).show(align='left', x=x, y=y)";
    py.eval(code, None, Some(&locals))?;
    Ok(())
}
