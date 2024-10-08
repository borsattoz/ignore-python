use pyo3::{prelude::*, types::PyAny};

fn register_child_module<'a>(parent_module: &'a Bound<'a, PyModule>, name: &'a str) -> PyResult<Bound<'a, PyModule>> {
    let child_module = PyModule::new_bound(parent_module.py(), name)?;

    parent_module.add_submodule(&child_module)?;

    let parent_module_name = parent_module.name()?;
    let mut parent_module_name = parent_module_name.to_str()?;

    if let Some(dot_index) = parent_module_name.find(".") {
        parent_module_name = &parent_module_name[..dot_index];
    }

    parent_module.py().import_bound("sys")?.getattr("modules")?.set_item(
        String::from(parent_module_name) + "." + name,
        &child_module
    )?;

    Ok(child_module)
}

struct PathBuf(std::path::PathBuf);

impl FromPyObject<'_> for PathBuf {
    fn extract_bound(path: &Bound<'_, PyAny>) -> PyResult<Self> {
        let builtins = PyModule::import_bound(path.py(), "builtins")?;

        let path = builtins
            .getattr("str")?
            .call((path,), None)?;
        let path: &str = path.extract()?;

        Ok(PathBuf(std::path::PathBuf::from(path)))
    }
}

impl IntoPy<PyObject> for PathBuf {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let pathlib = PyModule::import_bound(py, "pathlib").expect("no `pathlib`");
        let path = pathlib
            .getattr("Path").expect("no `pathlib.Path`")
            .call1((self.0,)).expect("wrong call to `Path`");

        path.unbind()
    }
}

struct Path<'a>(&'a std::path::Path);

impl IntoPy<PyObject> for Path<'_> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let pathlib = PyModule::import_bound(py, "pathlib").expect("no `pathlib`");
        let path = pathlib
            .getattr("Path").expect("no `pathlib.Path`")
            .call1((self.0,)).expect("wrong call to `Path`");

        path.unbind()
    }
}

#[pymodule]
mod ignore {
    use std::io;

    use super::*;

    #[pyclass(extends=pyo3::exceptions::PyException)]
    struct Error(ignore_rust::Error);

    #[pyclass(extends=pyo3::exceptions::PyException)]
    struct IOError {
        #[pyo3(get)]
        errno: u32,

        strerror: String,

        #[pyo3(get)]
        filename: String,
    }

    #[pymethods]
    impl IOError {
        #[new]
        fn new(errno: u32, strerror: String, filename: String) -> Self {
            Self { errno, strerror, filename }
        }

        fn __str__(&self) -> String {
            self.strerror.clone()
        }
    }

    impl From<Error> for PyErr {
        fn from(error: Error) -> Self {
            match &error.0 {
                ignore_rust::Error::WithPath { path, err } => {
                    match err.as_ref() {
                        ignore_rust::Error::Io(io_error) => {
                            match io_error.kind() {
                                io::ErrorKind::NotFound => {
                                    Python::with_gil(|py| {
                                        let errno = py.import_bound("errno").expect("`errno` module")
                                            .getattr("ENOENT").expect("`errno.ENOENT` constant")
                                            .extract().expect("`int` value");
                                        let strerror = error.0.to_string();
                                        let filename = path.clone().into_os_string().into_string().expect("a path");

                                        PyErr::from_value_bound(Bound::new(py, IOError { errno, strerror, filename }).unwrap().into_any())
                                    })
                                },
                                _ => PyErr::new::<Error, _>(error.0.to_string())
                            }
                        }
                        _ => PyErr::new::<Error, _>(error.0.to_string())
                    }
                },
                _ => PyErr::new::<Error, _>(error.0.to_string())
            }
        }
    }

    impl From<ignore_rust::Error> for Error {
        fn from(other: ignore_rust::Error) -> Self {
            Self(other)
        }
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let overrides = register_child_module(m, "overrides")?;

        overrides.add_class::<overrides::OverrideBuilder>()?;
        overrides.add_class::<overrides::Override>()
    }

    #[pyclass]
    struct DirEntry(ignore_rust::DirEntry);

    #[pymethods]
    impl DirEntry {
        fn path(&self) -> Path {
            Path(self.0.path())
        }

        fn depth(&self) -> usize {
            self.0.depth()
        }
    }

    #[pyclass]
    struct WalkBuilder(ignore_rust::WalkBuilder);

    #[pymethods]
    impl WalkBuilder {
        #[new]
        fn new(path: PathBuf) -> PyResult<Self> {
            Ok(Self(ignore_rust::WalkBuilder::new(path.0)))
        }

        fn hidden<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.hidden(yes);

            slf
        }

        fn ignore<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.ignore(yes);

            slf
        }

        fn parents<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.parents(yes);

            slf
        }

        fn git_ignore<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.git_ignore(yes);

            slf
        }

        fn git_global<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.git_global(yes);

            slf
        }

        fn git_exclude<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.git_exclude(yes);

            slf
        }

        fn require_git<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.require_git(yes);

            slf
        }

        fn overrides<'a>(mut slf: PyRefMut<'a, Self>, overrides: overrides::Override) -> PyRefMut<'a, Self> {
            slf.0.overrides(overrides.0);

            slf
        }

        fn follow_links<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.follow_links(yes);

            slf
        }

        fn same_file_system<'a>(mut slf: PyRefMut<'a, Self>, yes: bool) -> PyRefMut<'a, Self> {
            slf.0.same_file_system(yes);

            slf
        }

        #[pyo3(signature = (depth=None))]
        fn max_depth<'a>(mut slf: PyRefMut<'a, Self>, depth: Option<usize>) -> PyRefMut<'a, Self> {
            slf.0.max_depth(depth);

            slf
        }

        fn add_custom_ignore_filename<'a>(mut slf: PyRefMut<'a, Self>, file_name: &str) -> PyRefMut<'a, Self> {
            slf.0.add_custom_ignore_filename(file_name);

            slf
        }

        fn add<'a>(mut slf: PyRefMut<'a, Self>, path: PathBuf) -> PyRefMut<'a, Self> {
            slf.0.add(path.0);

            slf
        }

        fn add_ignore(&mut self, path: PathBuf) -> PyResult<()> {
            if let Some(e) = self.0.add_ignore(path.0) {
                Err(Error(e).into())
            } else {
                Ok(())
            }
        }

        fn build(&self) -> Walk {
            Walk(self.0.build())
        }
    }

    #[pyclass]
    struct Walk(ignore_rust::Walk);

    #[pymethods]
    impl Walk {
        #[new]
        fn new(path: PathBuf) -> Self {
            Self(ignore_rust::Walk::new(path.0))
        }

        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Result<DirEntry, Error>> {
            slf.0.next()
                .map(|res| res
                    .map(|dent| DirEntry(dent))
                    .map_err(|e| Error(e)))
        }
    }

    mod overrides {
        use super::*;

        #[pyclass]
        #[derive(Clone)]
        pub struct Override(pub ignore_rust::overrides::Override);

        #[pyclass]
        pub struct OverrideBuilder(ignore_rust::overrides::OverrideBuilder);

        #[pymethods]
        impl OverrideBuilder {
            #[new]
            fn new(py: Python<'_>, path: &Bound<'_, PyAny>) -> Result<Self, PyErr> {
                let builtins = PyModule::import_bound(py, "builtins")?;

                let path = builtins
                    .getattr("str")?
                    .call1((path,))?;
                let path: &str = path
                    .extract()?;
                let path = std::path::Path::new(path);

                Ok(Self(ignore_rust::overrides::OverrideBuilder::new(path)))
            }

            fn build(&self) -> Result<Override, Error> {
                self.0.build()
                    .map(|o| Override(o))
                    .map_err(|e| Error(e))
            }

            fn add<'a>(mut slf: PyRefMut<'a, Self>, glob: &'a str) -> Result<PyRefMut<'a, Self>, Error> {
                match slf.0.add(glob) {
                    Ok(_) => Ok(slf),
                    Err(e) => Err(Error(e))
                }
            }
        }
    }
}
