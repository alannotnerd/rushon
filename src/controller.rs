use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
    process::Child,
    thread,
};

use colored::{Color, Colorize};

use crate::manifest::Manifest;

#[derive(Debug, Default)]
pub struct Controller {
    manifests: BTreeMap<String, Manifest>,
}

impl Controller {
    pub fn new(manifests: BTreeMap<String, Manifest>) -> Self {
        Self { manifests }
    }
    pub fn run(&mut self) {
        let mut handlers = vec![];
        for (name, manifest) in &self.manifests {
            let mut child = manifest.run();
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            let id = child.id();
            {
                let name = name.clone();
                if let Some(stdout) = stdout {
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    println!(
                                        "{}: {}",
                                        format!("{}({})", name, id).color(Color::TrueColor {
                                            r: (id % 64 % 4 * 50 + 55) as u8,
                                            g: (id % 64 / 4 % 4 * 50 + 55) as u8,
                                            b: (id % 64 / 16 % 4 * 50 + 55) as u8
                                        }),
                                        line
                                    )
                                }
                                Err(e) => eprintln!("{}: {}", name, e),
                            }
                        }
                    });
                }
            }

            {
                let name = name.clone();
                if let Some(stderr) = stderr {
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    println!(
                                        "{}: {}",
                                        format!("{}({})", name, id).on_color(Color::TrueColor {
                                            r: (id % 64 % 4 * 50 + 55) as u8,
                                            g: (id % 64 / 4 % 4 * 50 + 55) as u8,
                                            b: (id % 64 / 16 % 4 * 50 + 55) as u8
                                        }),
                                        line
                                    )
                                }
                                Err(e) => eprintln!("{}: {}", name, e),
                            }
                        }
                    });
                }
            }

            handlers.push(child);
        }

        for handler in handlers.iter_mut() {
            handler.wait().expect("failed to wait for child process");
        }
    }
}
