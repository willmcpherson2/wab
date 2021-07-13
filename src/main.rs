use wasmer::{imports, wat2wasm, Function, Instance, Module, NativeFunc, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> anyhow::Result<()> {
    let wasm_bytes = wat2wasm(
        br#"
(module
  (type $hello_t (func (param) (result)))

  (import "env" "hello" (func $hello (type $hello_t)))

  (func $run (type $hello_t)
    (call $hello))

  (export "run" (func $run)))
"#,
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    let module = Module::new(&store, wasm_bytes)?;

    fn hello() {
        println!("Hello, world!")
    }

    let import_object = imports! {
        "env" => {
            "hello" => Function::new_native(&store, hello),
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;

    run_func.call()?;

    Ok(())
}
