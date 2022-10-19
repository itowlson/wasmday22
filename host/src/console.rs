use std::{
    path::Path,
};

use anyhow::Context;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

wit_bindgen_wasmtime::import!({paths: ["../wit/console.wit"]});

struct ConsoleContext {
    wasi: WasiCtx,
    random_thing: crate::services::RandomThing,
    data: console::ConsoleData,
}

impl ConsoleContext {
    fn new() -> Self {
        Self {
            wasi: WasiCtxBuilder::new().build(),
            random_thing: crate::services::RandomThing,
            data: console::ConsoleData {},
        }
    }
}

#[derive(Clone)]
pub(crate) struct ConsoleHandler {
    name: String,
    wasm: Vec<u8>,
}

impl ConsoleHandler {
    pub(crate) fn load(wasm_path: &Path) -> anyhow::Result<Self> {
        let name = wasm_path.file_stem().and_then(|s| s.to_str()).unwrap_or("mystery-file").to_owned();
        let wasm = std::fs::read(&wasm_path).with_context(|| {
            format!("Failed loading console handler from {}", wasm_path.display())
        })?;
        Ok(Self { name, wasm })
    }

    pub(crate) fn handle_input(&self, input: &str) -> anyhow::Result<String> {
        let name = &self.name;

        let ctx = ConsoleContext::new();
        let engine = Engine::default();
        let mut store = Store::new(&engine, ctx);
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut ConsoleContext| &mut ctx.wasi)
            .with_context(|| format!("Setting up WASI for console handler {}", name))?;
        crate::services::random_thing::add_to_linker(&mut linker, |ctx| &mut ctx.random_thing)
            .with_context(|| format!("Setting up services [RandomThing] for console handler {}", name))?;
        let module = Module::new(&engine, &self.wasm)
            .with_context(|| format!("Creating Wasm module for console handler {}", name))?;
        let instance = linker
            .instantiate(&mut store, &module)
            .with_context(|| format!("Instantiating Wasm module for console handler {}", name))?;
        let handler_exec =
            console::Console::new(&mut store, &instance, |ctx| &mut ctx.data)
                .with_context(|| format!("Loading Wasm executor for console handler {}", name))?;
        let response = handler_exec.handle_console_input(&mut store, input).unwrap();
        Ok(response)
    }
}

// impl Debug for ConsoleHandler {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("ConsoleHandler").finish()
//     }
// }

// impl ParseFilter for CustomFilterParser {
//     fn parse(
//         &self,
//         _arguments: liquid_core::parser::FilterArguments,
//     ) -> liquid_core::Result<Box<dyn Filter>> {
//         Ok(Box::new(CustomFilter {
//             name: self.name.to_owned(),
//             wasm_store: self.wasm_store.clone(),
//             exec: self.exec.clone(),
//         }))
//     }

//     fn reflection(&self) -> &dyn liquid_core::FilterReflection {
//         self
//     }
// }

// const EMPTY: [liquid_core::parser::ParameterReflection; 0] = [];

// impl liquid_core::FilterReflection for CustomFilterParser {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn description(&self) -> &str {
//         ""
//     }

//     fn positional_parameters(&self) -> &'static [liquid_core::parser::ParameterReflection] {
//         &EMPTY
//     }

//     fn keyword_parameters(&self) -> &'static [liquid_core::parser::ParameterReflection] {
//         &EMPTY
//     }
// }

// struct CustomFilter {
//     name: String,
//     wasm_store: Arc<RwLock<Store<CustomFilterContext>>>,
//     exec: Arc<custom_filter::CustomFilter<CustomFilterContext>>,
// }

// impl Debug for CustomFilter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("CustomFilter")
//             .field("name", &self.name)
//             .finish()
//     }
// }

// impl Display for CustomFilter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.name)
//     }
// }

// impl Filter for CustomFilter {
//     fn evaluate(
//         &self,
//         input: &dyn ValueView,
//         _runtime: &dyn Runtime,
//     ) -> Result<liquid::model::Value, liquid_core::error::Error> {
//         let mut store = self
//             .wasm_store
//             .write()
//             .map_err(|e| liquid_err(format!("Failed to get custom filter Wasm store: {}", e)))?;
//         let input_str = self.liquid_value_as_string(input)?;
//         match self.exec.exec(&mut *store, &input_str) {
//             Ok(Ok(text)) => Ok(to_liquid_value(text)),
//             Ok(Err(s)) => Err(liquid_err(s)),
//             Err(trap) => Err(liquid_err(format!("{:?}", trap))),
//         }
//     }
// }

// impl CustomFilter {
//     fn liquid_value_as_string(&self, input: &dyn ValueView) -> Result<String, liquid::Error> {
//         let str = input.as_scalar().map(|s| s.into_cow_str()).ok_or_else(|| {
//             liquid_err(format!(
//                 "Filter '{}': no input or input is not a string",
//                 self.name
//             ))
//         })?;
//         Ok(str.to_string())
//     }
// }

// fn to_liquid_value(value: String) -> liquid::model::Value {
//     liquid::model::Value::Scalar(liquid::model::Scalar::from(value))
// }

// fn liquid_err(text: String) -> liquid_core::error::Error {
//     liquid_core::error::Error::with_msg(text)
// }
