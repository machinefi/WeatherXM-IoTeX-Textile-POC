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

use zk::avg;
use zk_methods::ZK_ID;

fn main() {
    let json_str = "[
        \"{\\\"data\\\":\\\"[{\\\\\\\"device_id\\\\\\\":\\\\\\\"e1f8a900-1dfb-11ed-9972-4f669f2d96bd\\\\\\\",\\\\\\\"qod_score\\\\\\\":0.9644833333333332},{\\\\\\\"device_id\\\\\\\":\\\\\\\"024bf2a0-1d59-11ed-960f-d7d4cf200cc9\\\\\\\",\\\\\\\"qod_score\\\\\\\":1.0},{\\\\\\\"device_id\\\\\\\":\\\\\\\"4e3f3aa0-bf57-11ed-95eb-b351f0b0cc44\\\\\\\",\\\\\\\"qod_score\\\\\\\":0.9937500000000001}]\\\", \\\"receipt_type\\\":\\\"Stark\\\"}\"
        ]";

    let (receipt, _) = avg(json_str.to_owned());
    // Verify receipt, panic if it's wrong
    receipt.verify(ZK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
