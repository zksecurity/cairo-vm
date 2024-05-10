mod utils;

use cairo_vm::{
    cairo_run::{cairo_run, CairoRunConfig},
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    types::layout_name::LayoutName,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    crate::utils::set_panic_hook();
}

// TODO: check why this is needed. Seems wasm-bindgen expects us to use
// `std::error::Error` even if it's not yet in `core`
macro_rules! wrap_error {
    ($xp: expr) => {
        $xp.map_err(|e| JsError::new(e.to_string().as_str()))
    };
}

#[wasm_bindgen(js_name = runCairoProgram)]
pub fn run_cairo_program() -> Result<String, JsError> {
    const PROGRAM_JSON: &[u8] = include_bytes!("../../../cairo_programs/array_sum.json");

    let mut hint_executor = BuiltinHintProcessor::new_empty();

    let cairo_run_config = CairoRunConfig {
        layout: LayoutName::all_cairo,
        relocate_mem: true,
        trace_enabled: true,
        ..Default::default()
    };

    let mut runner = wrap_error!(cairo_run(
        PROGRAM_JSON,
        &cairo_run_config,
        &mut hint_executor
    ))?;

    let mut buffer = String::new();

    wrap_error!(runner.vm.write_output(&mut buffer))?;

    log(buffer.as_str());

    Ok(buffer)
}
