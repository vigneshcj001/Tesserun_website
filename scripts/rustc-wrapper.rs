use std::{env, process::Command};

fn main() {
    let mut args = env::args_os().skip(1);
    let compiler = args.next().expect("Cargo must provide the compiler");
    let mut args: Vec<_> = args.collect();
    let wasm = args.iter().any(|arg| {
        let arg = arg.to_string_lossy();
        arg.starts_with("--target=wasm32") || arg.strip_prefix('@').is_some_and(|path| {
            std::fs::read_to_string(path).is_ok_and(|flags| flags.contains("--target") && flags.contains("wasm32-unknown-unknown"))
        })
    }) || args.windows(2).any(|pair| {
        pair[0] == "--target" && pair[1].to_string_lossy().starts_with("wasm32")
    });
    // Cargo omits target rustflags for native build scripts and proc macros
    // during a WASM build. Use the local GNU toolchain's bundled host CRT.
    if !wasm {
        args.extend([
            "-Clinker-flavor=ld".into(),
            "-Clink-self-contained=yes".into(),
            "-Cdlltool=llvm-dlltool.exe".into(),
        ]);
    }
    let status = Command::new(compiler).args(args).status().expect("Run rustc");
    std::process::exit(status.code().unwrap_or(1));
}
