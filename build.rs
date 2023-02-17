use std::error::Error;
use std::path::Path;
use std::process::Command;

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
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let lai_path = Path::new(&out_dir).join("bundled").join("lai");
    let lai_path_str = lai_path.to_string_lossy();

    if !lai_path.exists() {
        Command::new("git")
            .args(&["clone", LAI_GITHUB_URL, &lai_path_str])
            .status()
            .unwrap();
    }

    let sources = SOURCES
        .iter()
        .map(|file| format!("{lai_path_str}/{file}"))
        .collect::<Vec<_>>();

    // NOTE: The build is configured for kernel enviornments.
    cc::Build::new()
        .files(sources)
        .include(format!("{lai_path_str}/include"))
        .flag("-fno-stack-protector")
        // The mmx and sse features determine support for SIMD instructions, which can often speed up
        // programs significantly. However, using the large SIMD registers in OS kernels leads to
        // performance problems.
        .flag("-mno-sse")
        .flag("-mno-mmx")
        // Inform the compiler that the target does not have floating-point hardware.
        .flag("-msoft-float")
        // Disable stack pointer optimization called the “red zone”, because it would cause
        // stack corruptions otherwise.
        .flag("-mno-red-zone")
        .flag("-fno-builtin")
        .flag("-nostdlib")
        .flag("-ffreestanding")
        .compile("lai");

    Ok(())
}
