use std::{error::Error, path::PathBuf, process::Command};

// lai/meson.build - sources
const SOURCES: &[&str] = &[
    "core/error.c",
    "core/eval.c",
    "core/exec.c",
    "core/exec-operand.c",
    "core/libc.c",
    "core/ns.c",
    "core/object.c",
    "core/opregion.c",
    "core/os_methods.c",
    "core/variable.c",
    "core/vsnprintf.c",
    "helpers/pc-bios.c",
    "helpers/pci.c",
    "helpers/resource.c",
    "helpers/sci.c",
    "helpers/pm.c",
    "drivers/ec.c",
    "drivers/timer.c",
];

const LAI_GITHUB_URL: &str = "https://github.com/managarm/lai";

fn main() -> Result<(), Box<dyn Error>> {
    let lai_path_str = String::from("bundled/lai");

    if !PathBuf::from(lai_path_str.clone()).exists() {
        // If we have not already downloaded the source, do so now.
        Command::new("git")
            .args(&["clone", LAI_GITHUB_URL, &lai_path_str])
            .status()
            .unwrap();
    }

    let sources = SOURCES
        .iter()
        .map(|file| format!("{lai_path_str}/{file}"))
        .collect::<Vec<_>>();

    cc::Build::new()
        .files(sources)
        .include(format!("{lai_path_str}/include"))
        .flag("-fno-stack-protector")
        .compile("lai");

    Ok(())
}
