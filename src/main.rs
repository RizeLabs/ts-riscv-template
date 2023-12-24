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

use risc0_zkvm::{default_prover, ExecutorEnv};
use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};
use std::fs;
use std::fs::File;
use serde::{Deserialize, Serialize};
// use serde_json::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomInputs {
    pub function_name: String,
    pub param: i64,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct Config {
    entrypoint: String,
    argumentTypes: Vec<String>,
    returnTypes: Vec<String>,
}

fn run_guest(iters: i64) -> i32 {

    let wasm_file_path = "wasm/run.wasm";

    let wasm = match fs::read(wasm_file_path) {
        Ok(bytes) => bytes,
        Err(err) => { 
            println!("Error reading file: {:?}", err);
            Vec::<u8>::new()
         }
    };

    let mut file = File::open("config.json").expect("Failed to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read the file");

    let config: Config = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let entrypoint = vec![config.entrypoint];
    let argument_types = config.argumentTypes;
    let return_types = config.returnTypes;

    let custom_inputs = CustomInputs {
        function_name: String::from("run"),
        param: iters,
        wasm: wasm,
    };

    println!("custom_inputs: {:?}", custom_inputs);

    let env: ExecutorEnv<'_> = ExecutorEnv::builder()
        .write(&custom_inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove_elf(env, WASM_INTERP_ELF).unwrap();

    receipt.verify(WASM_INTERP_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    let result: i32 = receipt.journal.decode().unwrap();

    result
}

fn main() {
    let fib_iters: i64 = 10;
    let _ = run_guest(fib_iters);
}