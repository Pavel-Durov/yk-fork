//! A tool to run a C test under gdb.

use clap::Parser;
use std::{env, path::PathBuf, process::Command};
use tempfile::TempDir;
use tests::{EXTRA_LINK, mk_compiler};
use ykbuild::ykllvm_bin;

/// Run a C test under gdb.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The test to attach gdb to.
    test_file: PathBuf,

    /// Don't immediately run the program.
    #[arg(short = 'n', long)]
    wait_at_prompt: bool,

    /// Pass all arguments after `--` directly to GDB.
    #[arg(last = true, required = false)]
    gdb_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let md = env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_path = [&md, "c", (args.test_file.to_str().unwrap())]
        .iter()
        .collect::<PathBuf>();
    let tempdir = TempDir::new().unwrap();

    // Compile the test.
    //
    // Some tests expect to have extra objects linked.
    let extra_objs = EXTRA_LINK
        .get(&test_path.to_str().unwrap())
        .unwrap_or(&Vec::new())
        .iter()
        .map(|e| e.generate_obj(tempdir.path()))
        .collect::<Vec<PathBuf>>();

    let binstem = PathBuf::from(args.test_file.file_stem().unwrap());
    let binpath = [tempdir.path(), &binstem].iter().collect::<PathBuf>();
    let mut cmd = mk_compiler(
        ykllvm_bin("clang").as_path(),
        &binpath,
        &test_path,
        &extra_objs,
        true,
        None,
    );
    if !cmd.spawn().unwrap().wait().unwrap().success() {
        panic!("compilation failed");
    }

    // Now we have a test binary in a temporary directory, prepare an invocation of gdb, setting
    // environment variables as necessary.
    let mut gdb = Command::new("gdb");
    gdb.arg(&binpath);

    if !args.wait_at_prompt {
        gdb.args(["-ex", "run"]);
    }

    // Pass all GDB-specific arguments after '--'
    if !args.gdb_args.is_empty() {
        for gdb_arg in &args.gdb_args {
            gdb.arg(gdb_arg);
        }
    }
    // Run gdb!
    gdb.spawn().expect("failed to spawn gdb").wait().unwrap();
}
