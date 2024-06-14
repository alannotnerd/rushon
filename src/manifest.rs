use std::{
    collections::BTreeMap,
    path::PathBuf,
    process::{Child, Stdio},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum RetryPolicy {
    Always,
    Never,
    OnFail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub env: Option<BTreeMap<String, String>>,
    pub cwd: Option<PathBuf>,
    pub program: PathBuf,
    pub args: Option<Vec<String>>,
    pub deps: Option<Vec<String>>,
    pub retry: Option<RetryPolicy>,
}

impl Manifest {
    pub fn retry_policy(&self) -> RetryPolicy {
        self.retry.unwrap_or(RetryPolicy::Never)
    }

    pub fn run(&self) -> Child {
        let mut cmd = std::process::Command::new(&self.program);

        if let Some(ref args) = self.args {
            cmd.args(args);
        }

        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        if let Some(ref env) = self.env {
            for (k, v) in env {
                cmd.env(k, v);
            }
        }
        cmd.spawn().expect("failed to start process")
    }
}
