//! Build program to generate a program which runs all the testsuites.
//!
//! By generating a separate `#[test]` test for each file, we allow cargo test
//! to automatically run the files in parallel.

use std::env;
use std::fs::{read_dir, DirEntry, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rustc-link-search=native=./wasmtime-runtime");
    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set"));
    let mut out = File::create(out_dir.join("wast_testsuite_tests.rs"))
        .expect("error generating test source file");

    test_directory(&mut out, "misc_testsuite").expect("generating tests");
    test_directory(&mut out, "spec_testsuite").expect("generating tests");
}

fn test_directory(out: &mut File, testsuite: &str) -> io::Result<()> {
    let mut dir_entries: Vec<_> = read_dir(testsuite)
        .expect("reading testsuite directory")
        .map(|r| r.expect("reading testsuite directory entry"))
        .filter(|dir_entry| {
            let p = dir_entry.path();
            if let Some(ext) = p.extension() {
                // Only look at wast files.
                if ext == "wast" {
                    // Ignore files starting with `.`, which could be editor temporary files
                    if let Some(stem) = p.file_stem() {
                        if let Some(stemstr) = stem.to_str() {
                            if !stemstr.starts_with('.') {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        })
        .collect();

    dir_entries.sort_by_key(|dir| dir.path());

    writeln!(
        out,
        "mod {} {{",
        Path::new(testsuite)
            .file_stem()
            .expect("testsuite filename should have a stem")
            .to_str()
            .expect("testsuite filename should be representable as a string")
            .replace("-", "_")
    )?;
    writeln!(
        out,
        "    use super::{{native_isa, Path, WastContext, Compiler}};"
    )?;
    for dir_entry in dir_entries {
        write_testsuite_tests(out, dir_entry, testsuite)?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn write_testsuite_tests(out: &mut File, dir_entry: DirEntry, testsuite: &str) -> io::Result<()> {
    let path = dir_entry.path();
    let stemstr = path
        .file_stem()
        .expect("file_stem")
        .to_str()
        .expect("to_str");

    writeln!(out, "    #[test]")?;
    if ignore(testsuite, stemstr) {
        writeln!(out, "    #[ignore]")?;
    }
    writeln!(
        out,
        "    fn {}() {{",
        avoid_keywords(&stemstr.replace("-", "_"))
    )?;
    writeln!(out, "        let isa = native_isa();")?;
    writeln!(out, "        let compiler = Compiler::new(isa);")?;
    writeln!(
        out,
        "        let mut wast_context = WastContext::new(Box::new(compiler));"
    )?;
    writeln!(out, "        wast_context")?;
    writeln!(out, "            .register_spectest()")?;
    writeln!(
        out,
        "            .expect(\"instantiating \\\"spectest\\\"\");"
    )?;
    writeln!(out, "        wast_context")?;
    write!(out, "            .run_file(Path::new(\"")?;
    // Write out the string with escape_debug to prevent special characters such
    // as backslash from being reinterpreted.
    for c in path.display().to_string().chars() {
        write!(out, "{}", c.escape_debug())?;
    }
    writeln!(out, "\"))")?;
    writeln!(out, "            .expect(\"error running wast file\");",)?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

/// Rename tests which have the same name as Rust keywords.
fn avoid_keywords(name: &str) -> &str {
    match name {
        "if" => "if_",
        "loop" => "loop_",
        "type" => "type_",
        "const" => "const_",
        "return" => "return_",
        other => other,
    }
}

/// Ignore tests that aren't supported yet.
fn ignore(testsuite: &str, name: &str) -> bool {
    if cfg!(windows) {
        return match (testsuite, name) {
            ("spec_testsuite", "address") => true,
            ("spec_testsuite", "align") => true,
            ("spec_testsuite", "call") => true,
            ("spec_testsuite", "call_indirect") => true,
            ("spec_testsuite", "conversions") => true,
            ("spec_testsuite", "elem") => true,
            ("spec_testsuite", "fac") => true,
            ("spec_testsuite", "func_ptrs") => true,
            ("spec_testsuite", "globals") => true,
            ("spec_testsuite", "i32") => true,
            ("spec_testsuite", "i64") => true,
            ("spec_testsuite", "f32") => true,
            ("spec_testsuite", "f64") => true,
            ("spec_testsuite", "if") => true,
            ("spec_testsuite", "imports") => true,
            ("spec_testsuite", "int_exprs") => true,
            ("spec_testsuite", "linking") => true,
            ("spec_testsuite", "memory_grow") => true,
            ("spec_testsuite", "memory_trap") => true,
            ("spec_testsuite", "resizing") => true,
            ("spec_testsuite", "select") => true,
            ("spec_testsuite", "skip-stack-guard-page") => true,
            ("spec_testsuite", "start") => true,
            ("spec_testsuite", "traps") => true,
            ("spec_testsuite", "unreachable") => true,
            ("spec_testsuite", "unwind") => true,
            ("misc_testsuite", "misc_traps") => true,
            ("misc_testsuite", "stack_overflow") => true,
            (_, _) => false,
        };
    }
    false
}
