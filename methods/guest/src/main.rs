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
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputValues {
    LongInteger(i64),
    ShortInteger(i32),
    LongUnsignedInteger(u64),
    ShortUnsignedInteger(u32),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuestInputs  {
    pub function_name: String,
    pub argument_types: Vec<String>,
    pub return_types: Vec<String>,
    pub function_inputs: Vec<InputValues>,
    pub wasm: Vec<u8>,
}

pub fn main() {
    let engine = Engine::default();
    let inputs: GuestInputs = env::read();

    let wasm: Vec<u8> = inputs.wasm;

    // for now first value is always i64
    let input: i64 = match inputs.function_inputs.get(0).unwrap() {
        InputValues::LongInteger(i) => *i,
        _ => 0,
    };

    let function_name: String = inputs.function_name;

    env::log(&format!("input: {:?}", input));
    env::log(&format!("function_name: {:?}", function_name));

    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = i64;

    let linker = <Linker<HostState>>::new(&engine);
    let mut store = Store::new(&engine, 42);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let function_instance = instance // take in i64 and return i64
        .get_typed_func::<i64, i64>(&store, &function_name)
        .expect("Failed to get typed_func");
    let res = function_instance.call(&mut store, input).expect("Failed to call");
    env::log(&format!("fib {} - {}", input, res));
    env::commit(&res);
}
