use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    // todo: use enviornment variable to find the lai directory.
    let lai_path = String::from("bundled/lai");
    let sources = SOURCES
        .iter()
        .map(|file| format!("{lai_path}/{file}"))
        .collect::<Vec<_>>();

    cc::Build::new()
        .files(sources)
        .include(format!("{lai_path}/include"))
        .flag("-fno-stack-protector")
        .compile("lai");

    Ok(())
}
