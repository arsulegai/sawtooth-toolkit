// Copyright 2020 Walmart Inc.
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

use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use std::fs::{read_to_string, File};
use std::io::Write;

pub(crate) struct KeysConstruct {
    format: String,
    file: String,
}

impl KeysConstruct {
    pub(crate) fn new(format: String, file: String) -> KeysConstruct {
        KeysConstruct { format, file }
    }

    pub(crate) fn convert(&self, format: String, file: String) -> bool {
        debug!("Requested to convert to {}", format);

        if &format == &self.format {
            info!("Requested format matches the input format!");
            return false;
        }
        if &format == "raw" {
            info!("Not Supported! In Progress");
            return true;
        }

        // Read the file and convert to the expected form
        let mut input_file_string = match read_to_string(&self.file) {
            Ok(string) => string,
            Err(err) => {
                error!("{:?}", err);
                return false;
            }
        };
        input_file_string = input_file_string.trim().into();

        let secp256k1_pvt_key: Secp256k1PrivateKey;
        match self.format.as_str() {
            "raw" => {
                secp256k1_pvt_key = match Secp256k1PrivateKey::from_hex(&input_file_string) {
                    Ok(success) => success,
                    Err(err) => {
                        error!("{:?}", err);
                        return false;
                    }
                };
                let output = match secp256k1_pvt_key.to_pem() {
                    Ok(success) => success,
                    Err(err) => {
                        error!("{:?}", err);
                        return false;
                    }
                };
                debug!("{:?}", output);
                return self.write_to_file(output, file);
            }
            _ => {
                error!("Unexpected flow!");
                return false;
            }
        };
    }

    fn write_to_file(&self, contents: String, file: String) -> bool {
        let mut file_var = match File::create(file) {
            Ok(created) => created,
            Err(err) => {
                error!("{:?}", err);
                return false;
            }
        };
        match file_var.write_all(contents.into_bytes().as_ref()) {
            Ok(_) => return true,
            Err(err) => {
                error!("{:?}", err);
                return false;
            }
        }
    }
}
