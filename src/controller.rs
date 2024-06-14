use std::{
    collections::{BTreeMap, VecDeque},
    io::{BufRead, BufReader},
    process::Child,
    thread,
};

use colored::{Color, Colorize};

use crate::manifest::{self, Manifest};

#[derive(Debug, Default)]
pub struct Controller {
    manifests: BTreeMap<String, Manifest>,
}

impl Controller {
    pub fn new(manifests: BTreeMap<String, Manifest>) -> Self {
        Self { manifests }
    }

    pub fn launch_one(name: &str, manifest: &Manifest) -> Child {
        let mut child = manifest.run();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let id = child.id();
        {
            let name = name.to_owned();
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
            let name = name.to_owned();
            if let Some(stderr) = stderr {
                thread::spawn(move || {
                    let reader = BufReader::new(stderr);
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

        child
    }

    pub fn run(&mut self) {
        let mut waitlist = self
            .manifests
            .iter()
            .map(|(k, v)| (k.clone(), v))
            .collect::<VecDeque<_>>();
        let mut handlers = BTreeMap::new();
        loop {
            while let Some((name, manifest)) = waitlist.pop_front() {
                let child = Self::launch_one(&name, manifest);
                handlers.insert(name.clone(), child);
            }

            for (name, handle) in handlers.iter_mut() {
                let ret = handle.try_wait().expect("failed to wait for child process");
                if let Some(ret) = ret {
                    if self.manifests[name].retry_policy() == manifest::RetryPolicy::Always {
                        eprintln!("{}: exited with {}, trying restart", name, ret);
                        waitlist.push_back((name.clone(), &self.manifests[name]));
                    }
                }
            }

            thread::sleep(std::time::Duration::from_millis(5000));
        }
    }
}
