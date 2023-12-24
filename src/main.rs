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
use std::fs::File;
use std::fs;
use std::io::Read; 
use serde::{Deserialize, Serialize, de::value};
use serde_json::Result;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuestInputs  {
    pub function_name: String,
    pub argument_types: Vec<String>,
    pub return_types: Vec<String>,
    pub function_inputs: Vec<InputValues>,
    pub wasm: Vec<u8>,
}

// for now only supporting u32, u64, i32, i64 because these values are more needed computationally
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputValues {
    LongInteger(i64),
    ShortInteger(i32),
    LongUnsignedInteger(u64),
    ShortUnsignedInteger(u32),
}

#[derive(Debug, Deserialize)]
struct Config {
    argument_types: Vec<String>,
    return_types: Vec<String>,
    inputs: Vec<String>,
}

fn run_guest() {

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

    let argument_types = config.argument_types;
    let return_types = config.return_types;
    let inputs = config.inputs;
    let mut function_inputs: Vec<InputValues> = Vec::new();

    for (index, value) in argument_types.iter().enumerate() {
        let input_type = value.to_string();

        match input_type.as_str() {
            "i32" => function_inputs.push(InputValues::ShortInteger(inputs[index].parse::<i32>().unwrap())),
            "i64" => function_inputs.push(InputValues::LongInteger(inputs[index].parse::<i64>().unwrap())),
            "u32" => function_inputs.push(InputValues::ShortUnsignedInteger(inputs[index].parse::<u32>().unwrap())),
            "u64" => function_inputs.push(InputValues::LongUnsignedInteger(inputs[index].parse::<u64>().unwrap())),
            _ => println!("Invalid input type"),
        }
    }

    println!("argument_types: {:?}", argument_types);
    println!("return_types: {:?}", return_types);

    let custom_inputs = GuestInputs {
        function_name: String::from("run"), // entrypoint is run for now
        argument_types,
        return_types,
        function_inputs,
        wasm,
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

    // let result: i32 = receipt.journal.decode().unwrap();
    
}

fn main() {
    run_guest();
}