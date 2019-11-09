use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::vm::VM;

/// Core struct for the REPL
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    /// Accepts a hexadecimal string and returns a Vec<u8>
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(results)
    }

    pub fn run(&mut self) {
        println!("Firefaith 0.0.1");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");

            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                "quit()" => {
                    std::process::exit(0);
                },
                "history()" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                "program()" => {
                    println!("Current instructions as bytecode:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                },
                "registers()" => {
                    println!("Registers:");
                    println!("{:#?}", self.vm.registers);
                },
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        },
                        Err(e) => {
                            println!("Unable to decode hex string. \
                                Please enter 4 hex bytes.");
                        }
                    };

                    self.vm.run_once();
                }
            }
        }
    }
}
