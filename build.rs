use std::fs;

#[cfg(windows)]
fn is_nixos() -> bool { false }

#[cfg(unix)]
fn is_nixos() -> bool {
    let Ok(content) = fs::read_to_string("/etc/os-release") else { return false; };


    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("ID=") {
            return rest.trim() == "nixos";
        }
    }

    return false;
}

fn main() {
    println!(
        "cargo:rustc-env=TARGET={}{}",
        build_target::target().triple,
        if is_nixos() { "_nixos" } else { "" } // Make nixos a different triple - it links to store paths which doesn't work on other distros
    );
}
