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
// #![no_std]

use risc0_zkvm::guest::env;
use serde::Deserialize;
use serde_json::Value as JsonValue;

risc0_zkvm::guest::entry!(main);

#[derive(Deserialize)]
struct Device {
    device_id: String,
    qod_score: f64,
}

pub fn main() {

    let input: String = env::read();
    // env::log(&format!("input: {:?}", input));

    let input_v: JsonValue = serde_json::from_str(&input).unwrap();
    let item_str = input_v.as_array().unwrap()[0].as_str().unwrap();
    let v: JsonValue = serde_json::from_str(item_str).unwrap();

    // env::log(&format!("input v: {:?}", v));
    let devices: Vec<Device> = serde_json::from_str(v["data"].as_str().unwrap()).unwrap();
    let sum: f64 = devices.iter().map(|x| x.qod_score).sum();
    let avg = sum / (devices.len() as f64);

    env::commit(&format!("{}", avg));
}
