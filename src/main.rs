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

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use crate::keys::KeysConstruct;
use clap::{App, AppSettings, Arg, SubCommand};
#[cfg(test)]
use flexi_logger::FlexiLoggerError;
use flexi_logger::{DeferredNow, LevelFilter, LogSpecBuilder, Logger, Record};
use std::process::exit;

mod keys;

pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", record.args(),)
}

fn main() {
    let matches = App::new("Sawtooth CLI Toolkit")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("keys")
                .subcommand(
                    SubCommand::with_name("convert")
                        .about("convert from one form to another")
                        .arg(
                            Arg::with_name("inform")
                                .long("inform")
                                .default_value("raw")
                                .possible_values(&["raw", "pem"])
                                .help("input file format")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("in")
                                .long("in")
                                .help("input file name")
                                .default_value("key.pub")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("outform")
                                .long("outform")
                                .help("output file format")
                                .default_value("pem")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("out")
                                .long("out")
                                .help("output file name")
                                .default_value("key.pem")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("private")
                                .long("private")
                                .help("is this a private key")
                                .default_value("true")
                                .possible_values(&["true", "false"])
                                .takes_value(true),
                        ),
                )
                .about("toolkit for the keys")
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    let log_level = LevelFilter::Trace;
    let mut log_spec_builder = LogSpecBuilder::new();
    log_spec_builder.default(log_level);
    match Logger::with(log_spec_builder.build())
        .format(log_format)
        .log_target(flexi_logger::LogTarget::StdOut)
        .start()
    {
        Ok(_) => {}
        Err(err) => panic!("Failed to start the logger: {}", err),
    };

    if let Some(matches) = matches.subcommand_matches("keys") {
        if matches.subcommand_matches("convert").is_none() {
            error!("Command not supported!");
            exit(1);
        }

        if let Some(matches) = matches.subcommand_matches("convert") {
            let input_format = matches.value_of("inform").unwrap_or("raw");
            let input_file = matches.value_of("in").unwrap_or("key.pub");
            let output_format = matches.value_of("outform").unwrap_or("pem");
            let output_file = matches.value_of("out").unwrap_or("key.pem");
            let private = matches.value_of("private").unwrap_or("true");
            let is_private = match private {
                "true" => true,
                "false" => false,
                _ => false,
            };

            if !is_private {
                info!("Not Supported! Feature TODO");
                exit(0);
            }

            let keys_construct =
                KeysConstruct::new(input_format.to_string(), input_file.to_string());
            if !keys_construct.convert(output_format.to_string(), output_file.to_string()) {
                error!("Failed!");
                exit(1);
            }
        }
    }

    info!("Execution complete!");
}
