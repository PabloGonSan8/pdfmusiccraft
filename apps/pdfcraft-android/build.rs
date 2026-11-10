//! Compiles `java/` (the immersive full-screen helper) into `classes.dex` for the Android build:
//! `javac` from `JAVA_HOME` against the SDK's `android.jar`, then the SDK's `d8`. Other targets get
//! an empty file and never load it.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=java");
    println!("cargo:rerun-if-env-changed=ANDROID_HOME");
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap_or_default());
    let dex = out.join("classes.dex");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
        let _ = std::fs::write(&dex, []);
        return;
    }
    if let Err(e) = build_dex(&out) {
        println!("cargo:warning=no immersive helper (the system bars stay): {e}");
        let _ = std::fs::write(&dex, []);
    }
}

fn build_dex(out: &Path) -> Result<(), String> {
    let sdk = PathBuf::from(std::env::var("ANDROID_HOME").map_err(|_| "ANDROID_HOME not set")?);
    let java = PathBuf::from(std::env::var("JAVA_HOME").map_err(|_| "JAVA_HOME not set")?);
    let jar = newest(&sdk.join("platforms"))?.join("android.jar");
    let tools = newest(&sdk.join("build-tools"))?;
    let d8 = if cfg!(windows) { tools.join("d8.bat") } else { tools.join("d8") };
    let classes = out.join("classes");
    let _ = std::fs::remove_dir_all(&classes);
    std::fs::create_dir_all(&classes).map_err(|e| e.to_string())?;
    let src = Path::new("java/org/pdfcraft/music/Immersive.java");
    run(Command::new(java.join("bin").join("javac")).args(["--release", "8", "-nowarn", "-classpath"]).arg(&jar).arg("-d").arg(&classes).arg(src))?;
    let class = classes.join("org/pdfcraft/music/Immersive.class");
    run(Command::new(d8).arg("--lib").arg(&jar).arg("--min-api").arg("26").arg("--output").arg(out).arg(class))
}

/// The last entry of `dir` by name (the newest platform or build-tools).
fn newest(dir: &Path) -> Result<PathBuf, String> {
    let mut all: Vec<PathBuf> =
        std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?.filter_map(Result::ok).map(|e| e.path()).collect();
    all.sort();
    all.pop().ok_or_else(|| format!("{} is empty", dir.display()))
}

fn run(cmd: &mut Command) -> Result<(), String> {
    let o = cmd.output().map_err(|e| format!("{cmd:?}: {e}"))?;
    if o.status.success() { Ok(()) } else { Err(format!("{cmd:?}: {}", String::from_utf8_lossy(&o.stderr))) }
}
