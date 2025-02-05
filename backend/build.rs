use std::{path::Path, process::Command};

#[allow(unused_macros)]
macro_rules! cargo_dbg {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");
    println!("cargo::rerun-if-changed=src/templates");

    minijinja_embed::embed_templates!("src/templates");

    std::fs::remove_dir_all("build").unwrap_or_default();

    Command::new("bun")
        .args([
            "run",
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/tailwind.css",
            "-o",
            "build/index.css",
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");

    Command::new("bun")
        .args([
            "build",
            "--minify",
            "--outdir=build",
            "--entry-naming",
            "[name].[ext]", //"[name].[hash].[ext]",
            "--asset-naming",
            "[name].[ext]", //"[name].[hash].[ext]",
            "./assets/scripts/index.ts",
        ])
        .status()
        .expect("failed to run bun");

    make_files_public("build", "build");
    make_files_public("assets/static", "assets/static");
    std::fs::remove_file("build/index.css").unwrap_or_default();
}

fn make_files_public(from_dir: &str, strip_dir: &str) {
    for entry in
        std::fs::read_dir(from_dir).expect(format!("failed to read dir `{}`", from_dir).as_str())
    {
        let entry = entry.expect("failed to read entry");

        if entry.file_type().unwrap().is_dir() {
            make_files_public(entry.path().to_str().unwrap(), strip_dir);
        } else {
            let path = entry.path();
            cargo_dbg!("Path: {:?}", path);

            // let filename = path.file_name().unwrap().to_str().unwrap();

            let dest = Path::new("public").join(entry.path().strip_prefix(strip_dir).unwrap());
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }

            cargo_dbg!(
                "Src: {:?}, Dest: {:?}, From_dir: {:?}, Entry: {:?}",
                path,
                dest,
                from_dir,
                entry.path().strip_prefix(from_dir).unwrap()
            );
            std::fs::copy(path, dest).expect("failed to copy file");
        }
    }
}
