use std::{
    path::{Path, PathBuf},
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
    random_thing_provider_wasm: Option<Vec<u8>>,
}

impl ConsoleHandler {
    pub(crate) fn load(wasm_path: &Path, random_thing_provider: &Option<PathBuf>) -> anyhow::Result<Self> {
        let name = wasm_path.file_stem().and_then(|s| s.to_str()).unwrap_or("mystery-file").to_owned();
        let wasm = std::fs::read(&wasm_path).with_context(|| {
            format!("Failed loading console handler from {}", wasm_path.display())
        })?;
        let random_thing_provider_wasm = random_thing_provider.as_ref().map(|path|
            std::fs::read(path).unwrap()
        );
        Ok(Self { name, wasm, random_thing_provider_wasm })
    }

    pub(crate) fn handle_input(&self, input: &str) -> anyhow::Result<String> {
        let name = &self.name;

        let ctx = ConsoleContext::new();
        let engine = Engine::default();
        let mut store = Store::new(&engine, ctx);
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut ConsoleContext| &mut ctx.wasi)
            .with_context(|| format!("Setting up WASI for console handler {}", name))?;
        match &self.random_thing_provider_wasm {
            None =>
                crate::services::random_thing::add_to_linker(&mut linker, |ctx| &mut ctx.random_thing)
                    .with_context(|| format!("Setting up services [RandomThing] for console handler {}", name))?,
            Some(wasm) => {
                let rtp = Module::new(&engine, &wasm).unwrap();
                linker.module(&mut store, "random-thing", &rtp)
                    .with_context(|| format!("Linking Java module for console handler {}", name))?;
            }
        }
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
