use anyhow::Result;
use wasmtime::{component::ResourceTable, Config, Engine, Linker, Module, Store};
use wasmtime_wasi::preview2::{
    preview1::{add_to_linker_async, WasiPreview1Adapter, WasiPreview1View},
    WasiCtx, WasiCtxBuilder, WasiView,
};

struct WasiHostCtx {
    preview2_ctx: WasiCtx,
    preview2_table: ResourceTable,
    preview1_adapter: WasiPreview1Adapter,
}

impl WasiView for WasiHostCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.preview2_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.preview2_ctx
    }
}

impl WasiPreview1View for WasiHostCtx {
    fn adapter(&self) -> &WasiPreview1Adapter {
        &self.preview1_adapter
    }

    fn adapter_mut(&mut self) -> &mut WasiPreview1Adapter {
        &mut self.preview1_adapter
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Construct the wasm engine with async support enabled.
    let engine = {
        let mut config = Config::new();
        config.async_support(true);
        Engine::new(&config)?
    };

    // Add the WASI preview1 API to the linker (will be implemented in terms of
    // the preview2 API)
    let mut linker: Linker<WasiHostCtx> = Linker::new(&engine);
    add_to_linker_async(&mut linker)?;

    let mut store: Store<WasiHostCtx> = {
        // Add capabilities (e.g. filesystem access) to the WASI preview2 context here.
        let wasi_ctx = {
            let args = std::env::args().collect::<Vec<_>>();
            let envs = std::env::vars().collect::<Vec<_>>();
            WasiCtxBuilder::new()
                .args(&args)
                .envs(&envs)
                .inherit_stdio()
                .build()
        };
        let host_ctx = WasiHostCtx {
            preview2_ctx: wasi_ctx,
            preview2_table: ResourceTable::new(),
            preview1_adapter: WasiPreview1Adapter::new(),
        };

        Store::new(&engine, host_ctx)
    };

    let module = {
        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let wasm_path = format!("aluvm-test/target/wasm32-wasi/{profile}/aluvm-test.wasm");
        Module::from_file(&engine, wasm_path)?
    };
    let aluvm_test = linker
        .module_async(&mut store, "", &module)
        .await?
        // `_start`, which will call `main` after some initializations.
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?;

    aluvm_test.call_async(&mut store, ()).await?;

    Ok(())
}
