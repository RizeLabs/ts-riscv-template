# 📄 TS-RISCV-Template

This template allows anyone to write their arbitrary code logic in typescript and get it's execution proof via Risc0.

## 🏗️ How does it works
This template is configured with [assemblyscript](https://www.assemblyscript.org/) and [Risc0 starter template](https://github.com/risc0/risc0). You can write your code logic in typescript. For generating proof of computation you typescript code get compiled into wasm via assemblyscript and the wasm file run inside Risc0 with your custom inputs using [wasmi](https://crates.io/crates/wasmi) which is a webassembly interpreter .

## 🧑‍💻 Steps to use it
1. Write your custom ts scripts in `package/run.ts` file and make sure to use rust compatible types (for example instead of using number as a type you can use i32, i64 etc) and ignore the type error meanwhile.
2. Define your I/O types in `config.json`
```
{
    "argument_types": [ // input argument types of your ts function
        "i64" 
    ],
    "return_types": [ // output argument types of your ts function
        "i64"
    ],
    "inputs":[ // input for your ts function
        "10"
    ]
}
```
3. `chmod +x` for both the scripts (install.sh and compile.sh) please.
4. Build the project using `./scripts/install.sh`
5. Run `./scripts/compile.sh` to compile your ts logic into wasm via asc (assembly script)
6. `cargo run` to run your compiled wasm into RiscV environment via wasmi.

## 🖊️ Note
- This template is under work and hence not production ready and currently only support arguments and return types as `i32, i64, u32 and u64`. 

