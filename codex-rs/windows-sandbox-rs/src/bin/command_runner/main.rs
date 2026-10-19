#![cfg_attr(all(target_os = "windows", not(test)), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod win;

#[cfg(all(windows, target_vendor = "win7", target_arch = "x86_64"))]
#[path = "../win7_time.rs"]
mod win7_time;

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    win::main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("codex-command-runner is Windows-only");
}
