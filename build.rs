// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=['./package.json', './frontend']");

    if !PathBuf::from("./target/frontend").exists() {
        let build_frontend = Command::new("bun")
            .arg("run")
            .arg("build")
            .status()
            .expect("Cannot invoke frontend build process.");

        assert!(build_frontend.success(), "Frontend build failed.");
    } else {
        println!("cargo::warning=Skipping frontend building.")
    }
}
