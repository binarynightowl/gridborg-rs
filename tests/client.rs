use std::sync::Once;
use pyo3::prelude::*;
use gridborg_rs::gridborg_rs as gridborg;

static PY_INIT: Once = Once::new();

pub fn init_python() {
    PY_INIT.call_once(|| {
        pyo3::append_to_inittab!(gridborg);
        pyo3::prepare_freethreaded_python();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_as_string() {
        init_python();

        Python::with_gil(|py| {
            let gridborg_rs = py
                .import("gridborg_rs")
                .expect("import gridborg_rust failed");

            let sum_as_string = gridborg_rs
                .getattr("sum_as_string")
                .expect("getattr sum_as_string failed");

            let result: PyResult<String> = match sum_as_string.call1((3usize, 5usize)) {
                Ok(r) => r.extract(),
                Err(e) => Err(e),
            };

            let result = result.unwrap();
            assert_eq!(result, "8");
        });
    }

    #[test]
    fn test_gridborg_client_creation() {
        init_python();

        Python::with_gil(|py| {
            let gridborg_rs = py
                .import("gridborg_rs")
                .expect("import gridborg failed");

            let client_module = gridborg_rs
                .getattr("client")
                .expect("getattr client failed");
            let gridborg_client_class = client_module
                .getattr("GridborgClient")
                .expect("getattr GridborgClient failed");

            // Instantiate a client
            let client = gridborg_client_class
                .call1((
                    "127.0.0.1", // server
                    1234u16,     // control_port
                    1235u16,     // transport_channel_port
                    "testuser",  // username
                    "testpass",  // password
                ))
                .expect("failed to create GridborgClient");

            // Access command_tag property
            let command_tag: u64 = client
                .getattr("command_tag")
                .expect("getattr command_tag failed")
                .extract()
                .expect("extract command_tag failed");

            assert_eq!(command_tag, 0);
        });
    }
}
