use std::env;
use std::process::Command;
use std::str;

static CARGOENV: &str = "cargo:rustc-env=";

fn main() {
    let out = match env::var("BUILD_TAG") {
        Ok(v) => v,
        Err(_) => {
            // let time_c = Command::new("date")
            //     .args(&["+%Y%m%d"])
            //     .output()
            //     .unwrap();
            // let ts = str::from_utf8(&time_c.stdout).unwrap();
            let gitrev_c = Command::new("git")
                .args(&["rev-parse", "HEAD"])
                .output()
                .unwrap();
            let ver = str::from_utf8(&gitrev_c.stdout).unwrap();
            ver.to_string()
        }
    };

    println!("{}BUILD_TAG={}", CARGOENV, out);
}
