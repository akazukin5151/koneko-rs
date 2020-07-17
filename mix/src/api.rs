use pyo3::prelude::*;
use pyo3::types::PyDict;


pub struct Data<'a> {
    pub username: &'a str,
    pub password: &'a str,
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
    locals.set_item("username", data.username)?;
    locals.set_item("password", data.password)?;
    Ok(locals)
}

fn eval(py: Python, locals: &PyDict) -> PyResult<()> {
    let code = "api.myapi.start({'Username': username, 'Password': password})";
    py.eval(code, None, Some(&locals))?;
    let code = "json.dumps(api.myapi.illust_follow_request('private', 0))";
    let result: String = py.eval(code, None, Some(&locals))?.extract()?;
    println!("{}", result);
    Ok(())
}
