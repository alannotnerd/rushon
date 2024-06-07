use std::collections::BTreeMap;

use controller::Controller;
use manifest::Manifest;

mod controller;
mod manifest;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file = args[1].as_str();

    let manifest = std::fs::read_to_string(file).expect("failed to read manifest file");
    let manifest: BTreeMap<String, Manifest> =
        toml::from_str(&manifest).expect("failed to parse manifest file");

    let mut controller = Controller::new(manifest);
    controller.run();
}
