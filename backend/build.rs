use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");

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

    make_files_public("build");
    std::fs::remove_file("build/index.css").unwrap_or_default();
}

fn make_files_public(from_dir: &str) {
    for entry in
        std::fs::read_dir(from_dir).expect(format!("failed to read dir `{}`", from_dir).as_str())
    {
        let entry = entry.expect("failed to read entry");

        if entry.file_type().unwrap().is_dir() {
            make_files_public(entry.path().to_str().unwrap());
        } else {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let dest = format!("./public/{}", filename);

            std::fs::copy(path, dest).expect("failed to copy file");
        }
    }
}
