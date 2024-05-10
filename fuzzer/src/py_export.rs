use cairo_vm::{
    cairo_run::{cairo_run, CairoRunConfig},
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
};
use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python},
};
use std::panic;

create_exception!(
    cairo_vm_rs,
    PanicTriggered,
    PyException,
    "Raised when panic_unwind catches a panic during `cairo_run`"
);
create_exception!(
    cairo_vm_rs,
    VMError,
    PyException,
    "`cairo_run` raised a `CairoRunError`"
);

#[pyfunction]
fn cairo_run_dump_mem(json: String) -> PyResult<Vec<u8>> {
    let config = CairoRunConfig {
        relocate_mem: true,
        ..Default::default()
    };

    let result_no_panic = panic::catch_unwind(|| {
        cairo_run(
            json.as_bytes(),
            &config,
            &mut BuiltinHintProcessor::new_empty(),
        )
    })
    .map_err(|e| {
        PanicTriggered::new_err(format! {"Rust VM panicked! {:?}", e.downcast::<String>()})
    })?;

    let cairo_runner =
        result_no_panic.map_err(|e| VMError::new_err(format! {"VM error: {:?}", e.to_string()}))?;

    let mut memory_dump = Vec::new();
    for (i, memory_cell) in cairo_runner.relocated_memory.iter().enumerate() {
        match memory_cell {
            None => continue,
            Some(unwrapped_memory_cell) => {
                memory_dump.extend_from_slice(&(i as u64).to_le_bytes());
                memory_dump.extend_from_slice(&unwrapped_memory_cell.to_bytes_le());
            }
        }
    }
    Ok(memory_dump)
}

#[pymodule]
fn cairo_vm_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("PanicTriggered", py.get_type::<PanicTriggered>())?;
    m.add("VMError", py.get_type::<VMError>())?;
    m.add_function(wrap_pyfunction!(cairo_run_dump_mem, m)?)?;
    Ok(())
}
