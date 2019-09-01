use crate::PyTimestamp;
use derive_more::From;
use hedera::FileInfo;
use pyo3::prelude::*;
use try_from::TryInto;

#[pyclass(name = FileInfo)]
#[derive(From)]
pub struct PyFileInfo {
    inner: FileInfo,
}

#[pymethods]
impl PyFileInfo {
    #[getter]
    pub fn file_id(&mut self) -> PyResult<String> {
        Ok(self.inner.file_id.to_string())
    }

    #[getter]
    pub fn size(&mut self) -> PyResult<i64> {
        Ok(self.inner.size)
    }

    #[getter]
    pub fn deleted(&mut self) -> PyResult<bool> {
        Ok(self.inner.deleted)
    }

    #[getter]
    pub fn keys(&mut self) -> PyResult<Vec<String>> {
        Ok(self.inner.keys.iter().map(|key| key.to_string()).collect())
    }

    #[getter]
    pub fn expiration_time(&self) -> PyResult<PyTimestamp> {
        self.inner.expiration_time.try_into()
    }
}
