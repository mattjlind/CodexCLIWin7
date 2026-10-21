#[cfg(all(windows, target_vendor = "win7", target_arch = "x86_64"))]
#[path = "../../win7_time.rs"]
mod win7_time;

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    codex_windows_sandbox::setup_helper_main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("codex-windows-sandbox-setup is Windows-only");
}
