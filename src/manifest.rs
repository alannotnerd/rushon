use std::{
    collections::BTreeMap,
    path::PathBuf,
    process::{Child, Stdio},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub env: BTreeMap<String, String>,
    pub cwd: PathBuf,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub deps: Vec<String>,
}

impl Manifest {
    pub fn load(path: PathBuf) -> Self {
        let manifest = std::fs::read_to_string(path).expect("failed to read manifest file");
        toml::from_str(&manifest).expect("failed to parse manifest file")
    }

    pub fn run(&self) -> Child {
        let mut cmd = std::process::Command::new(&self.program);

        cmd.args(&self.args)
            .current_dir(&self.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for (k, v) in &self.env {
            cmd.env(k, v);
        }
        cmd.spawn().expect("failed to start process")
    }
}
