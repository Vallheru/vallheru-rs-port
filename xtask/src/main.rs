use std::{env, path::Path};

type DynError = Box<dyn std::error::Error>;
type XResult = Result<(), DynError>;
const FILES_ALL: [&str; 0] = [];

fn to_dyn_err<T>(e: T) -> DynError
where
    T: std::error::Error + 'static,
{
    Box::new(e)
}

#[allow(unused_macros)]
macro_rules! cargo_dbg {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn print_help() {
    eprintln!(
        "Tasks:
\trun\t\trun an application locally
\trelease\t\trelease and prepare the program distribution
\tbuild\t\tjust build all the components and put it in the `build` directory
\tpublish\t\tpublish all the built components from the `build` directory to the `public` dir"
    )
}

fn try_main() -> XResult {
    let task = env::args().nth(1);

    match task.as_deref() {
        Some("release") => release()?,
        Some("build") => build()?,
        Some("publish") => publish()?,
        Some("run") => run()?,
        _ => print_help(),
    }

    Ok(())
}

fn release() -> XResult {
    todo!()
}

fn run() -> XResult {
    build()?;
    publish()?;

    cmd_exec(
        "cargo",
        [
            "watch",
            "-q",
            "-c",
            "-w",
            "./src",
            "-x",
            "run --bin vallheru",
        ],
    )
}

fn publish() -> XResult {
    make_files_public(
        "build",
        "build",
        [
            "index.js",
            "index.css",
            "vallheru_wasm_bg.wasm",
            "vallheru_wasm.js",
        ],
    )?;
    make_files_public("assets/static", "assets/static", FILES_ALL)?;

    Ok(())
}

fn build() -> XResult {
    std::fs::create_dir_all("build").map_err(to_dyn_err)?;
    std::fs::remove_dir_all("build").map_err(to_dyn_err)?;

    build_css("build")?;
    compile_ts("build")?;
    build_wasm("build")?;

    Ok(())
}

fn build_css(artifact_dir: &str) -> XResult {
    cmd_exec(
        "bun",
        [
            "run",
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/tailwind.css",
            "-o",
            &format!(
                "{}/index.css",
                artifact_dir.strip_suffix("/").unwrap_or(artifact_dir)
            ),
            "--minify",
        ],
    )
}

fn compile_ts(artifact_dir: &str) -> XResult {
    cmd_exec(
        "bun",
        [
            "build",
            "--minify",
            "--outdir",
            artifact_dir.strip_suffix("/").unwrap_or(artifact_dir),
            "--entry-naming",
            "[name].[ext]", //"[name].[hash].[ext]",
            "--asset-naming",
            "[name].[ext]", //"[name].[hash].[ext]",
            "./assets/scripts/index.ts",
        ],
    )
}

fn cmd_exec<const N: usize>(binary: &str, args: [&str; N]) -> XResult {
    let output = std::process::Command::new(binary).args(args).output();

    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

fn build_wasm(artifact_dir: &str) -> XResult {
    cmd_exec(
        "cargo",
        [
            "build",
            "-p",
            "vallheru-wasm",
            "--target",
            "wasm32-unknown-unknown",
            "--artifact-dir",
            artifact_dir.strip_suffix("/").unwrap_or(artifact_dir),
            "--release",
            "-Z",
            "unstable-options",
        ],
    )?;

    cmd_exec(
        "wasm-bindgen",
        [
            "--target",
            "web",
            "--out-dir",
            artifact_dir.strip_suffix("/").unwrap_or(artifact_dir),
            "./build/vallheru_wasm.wasm",
        ],
    )
}

// TODO: Refactor all that shitty unwraps
fn make_files_public<const N: usize>(
    from_dir: &str,
    strip_dir: &str,
    files_list: [&str; N],
) -> XResult {
    for entry in
        std::fs::read_dir(from_dir).expect(format!("failed to read dir `{}`", from_dir).as_str())
    {
        let entry = entry.expect("failed to read entry");

        if entry.file_type().unwrap().is_dir() {
            make_files_public(entry.path().to_str().unwrap(), strip_dir, files_list)?;
        } else {
            let path = entry.path();
            cargo_dbg!("Path: {:?}", path);

            // let filename = path.file_name().unwrap().to_str().unwrap();

            let dest = Path::new("public").join(entry.path().strip_prefix(strip_dir).unwrap());
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }

            if !files_list.is_empty() && files_list.contains(&entry.file_name().to_str().unwrap()) {
                continue;
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

    Ok(())
}
