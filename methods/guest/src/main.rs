// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
#![allow(unused_imports)]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};
// use core::CustomInputs;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomInputs {
    pub function_name: String,
    pub param: i64,
    pub wasm: Vec<u8>,
}

pub fn main() {
    let engine = Engine::default();
    let inputs: CustomInputs = env::read();

    let wasm: Vec<u8> = inputs.wasm;
    let iters: i64 = inputs.param;
    let function_name: String = inputs.function_name;

    // env::log(&format!("wasm: {:?}", wasm));
    env::log(&format!("iters: {:?}", iters));
    env::log(&format!("function_name: {:?}", function_name));

    // Derived from the wasmi example: https://docs.rs/wasmi/0.29.0/wasmi/#example
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = i64;

    let linker = <Linker<HostState>>::new(&engine);
    let mut store = Store::new(&engine, 42);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let fib = instance // take in i32 and return i32
        .get_typed_func::<i64, i64>(&store, &function_name)
        .expect("Failed to get typed_func");
    let res = fib.call(&mut store, iters).expect("Failed to call");
    env::log(&format!("fib {} - {}", iters, res));
    env::commit(&res);
}
