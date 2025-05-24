// MIT License

// Copyright (c) 2024 - present Nicholas D. Crosbie

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Check Rust version
    check_rust_version();

    // Generate canonical tetranucleotides lookup table
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("canonical_kmers.rs");
    let mut file = BufWriter::new(File::create(&dest_path).unwrap());

    let k = 4; // We are pre-computing for k=4 (tetranucleotides)
    let bases = ['A', 'C', 'G', 'T'];

    // Start generating the map code - using phf_codegen instead of phf_map macro
    writeln!(
        &mut file,
        "use phf::Map;\n\nstatic CANONICAL_K_MERS: Map<&'static str, &'static str> = {{"
    )
    .unwrap();

    writeln!(
        &mut file,
        "    // Generated map of tetranucleotides to their canonical forms"
    )
    .unwrap();
    writeln!(
        &mut file,
        "    const DATA: phf::Map<&'static str, &'static str> = {{"
    )
    .unwrap();

    // Use phf_codegen to create a phf::Map
    let mut builder = phf_codegen::Map::new();

    // Iterate through all possible k-mers (4^k combinations)
    // For k=4, this is 4^4 = 256 combinations.
    let mut current_kmer_chars = vec![' '; k];
    for i0 in 0..4 {
        current_kmer_chars[0] = bases[i0];
        for i1 in 0..4 {
            current_kmer_chars[1] = bases[i1];
            for i2 in 0..4 {
                current_kmer_chars[2] = bases[i2];
                for &base3 in &bases {
                    current_kmer_chars[3] = base3;

                    let kmer: String = current_kmer_chars.iter().collect();
                    let rc_kmer = reverse_complement(&kmer);
                    let canonical = if kmer <= rc_kmer {
                        kmer.clone()
                    } else {
                        rc_kmer
                    };

                    // Add the k-mer -> canonical k-mer entry to the builder
                    builder.entry(kmer, &format!("\"{}\"", canonical));
                }
            }
        }
    }

    // Write the generated phf::Map code
    write!(&mut file, "{}", builder.build()).unwrap();

    // Close the map definition
    writeln!(&mut file, "    }};").unwrap();
    writeln!(&mut file, "    DATA").unwrap();
    writeln!(&mut file, "}};").unwrap();

    // Check if SQLite is installed
    let sqlite_installed = check_library("sqlite3");

    // Check if zstd is installed
    let zstd_installed = check_library("zstd");

    // Print error and abort if any library is missing
    if !sqlite_installed || !zstd_installed {
        let mut error_msg = "Error: Missing required libraries.\n".to_string();

        if !sqlite_installed {
            error_msg.push_str("SQLite3 development library not found. ");
            match env::consts::OS {
                "macos" => error_msg.push_str("Install with: brew install sqlite3\n"),
                "linux" => error_msg.push_str(
                    "Install with: apt install libsqlite3-dev or yum install sqlite-devel\n",
                ),
                _ => error_msg.push_str("Please install SQLite3 development libraries\n"),
            }
        }

        if !zstd_installed {
            error_msg.push_str("zstd library not found. ");
            match env::consts::OS {
                "macos" => error_msg.push_str("Install with: brew install zstd\n"),
                "linux" => error_msg.push_str(
                    "Install with: apt install libzstd-dev or yum install libzstd-devel\n",
                ),
                _ => error_msg.push_str("Please install zstd development libraries\n"),
            }
        }

        eprintln!("{}", error_msg);
        panic!("Build aborted due to missing dependencies");
    }

    // Add library search paths if needed
    if env::consts::OS == "macos" {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
    }
}

// Helper function to get the reverse complement of a DNA sequence
fn complement_base(base: char) -> char {
    match base.to_ascii_uppercase() {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => base,
    }
}

fn reverse_complement(sequence: &str) -> String {
    sequence.chars().rev().map(complement_base).collect()
}

fn check_rust_version() {
    // Define minimum required Rust version based on Rust edition in Cargo.toml
    // For Rust 2021 edition, minimum version is 1.56.0
    let min_version = (1, 56, 0);

    // Get the current Rust version
    let rustc_output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc. Make sure Rust is installed.");

    let version_str = String::from_utf8_lossy(&rustc_output.stdout);

    // Parse the version string (format: "rustc 1.xx.x (hash date)")
    let version_parts: Vec<&str> = version_str.split(' ').collect();
    if version_parts.len() < 2 {
        eprintln!("Failed to parse Rust version string: {}", version_str);
        panic!("Could not determine Rust version");
    }

    let version_nums: Vec<&str> = version_parts[1].split('.').collect();
    if version_nums.len() < 3 {
        eprintln!("Failed to parse Rust version number: {}", version_parts[1]);
        panic!("Could not determine Rust version");
    }

    let major: u32 = version_nums[0].parse().unwrap_or(0);
    let minor: u32 = version_nums[1].parse().unwrap_or(0);
    // The patch version might include additional info, so parse carefully
    let patch_str = version_nums[2].split('-').next().unwrap_or("0");
    let patch_str = patch_str.split('+').next().unwrap_or("0");
    let patch: u32 = patch_str.parse().unwrap_or(0);

    let current_version = (major, minor, patch);

    println!("Detected Rust version: {}.{}.{}", major, minor, patch);

    // Compare with minimum version
    if current_version < min_version {
        eprintln!(
            "Error: Rust version {}.{}.{} is required, but you have {}.{}.{}",
            min_version.0, min_version.1, min_version.2, major, minor, patch
        );

        match env::consts::OS {
            "macos" => eprintln!("Update with: rustup update stable"),
            "linux" => eprintln!("Update with: rustup update stable"),
            _ => eprintln!("Please update your Rust installation using rustup"),
        }

        panic!("Build aborted due to outdated Rust version");
    }
}

fn check_library(lib_name: &str) -> bool {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    println!("Checking for {} library on {} {}", lib_name, os, arch);

    match os {
        "linux" => {
            // On Linux, use pkg-config to check for libraries
            let output = Command::new("pkg-config")
                .args(["--exists", lib_name])
                .status()
                .map(|status| status.success())
                .unwrap_or(false);

            if output {
                return true;
            }

            // Fallback to checking common library locations
            let lib_paths = [
                "/usr/lib",
                "/usr/local/lib",
                "/lib",
                "/usr/lib/x86_64-linux-gnu",
                "/usr/lib/aarch64-linux-gnu",
            ];

            for path in &lib_paths {
                let lib_file = format!("{}/lib{}.so", path, lib_name);
                if Path::new(&lib_file).exists() {
                    return true;
                }
            }

            false
        }
        "macos" => {
            // Try using pkg-config first (if available on macOS)
            let pkg_config_result = Command::new("pkg-config")
                .args(["--exists", lib_name])
                .status()
                .map(|status| status.success())
                .unwrap_or(false);

            if pkg_config_result {
                return true;
            }

            // For SQLite specifically, try querying brew for the installation path
            if lib_name == "sqlite3" {
                if let Ok(output) = Command::new("brew").args(["--prefix", "sqlite"]).output() {
                    if output.status.success() {
                        return true;
                    }
                }
            }

            // Check if we can compile a simple test program that links with the library
            let temp_dir = env::temp_dir();
            let test_file = temp_dir.join("lib_test.c");
            let test_exe = temp_dir.join("lib_test");

            std::fs::write(
                &test_file,
                format!("#include <{}.h>\nint main() {{ return 0; }}", lib_name),
            )
            .ok();

            let compile_result = Command::new("cc")
                .args([
                    "-o",
                    test_exe.to_str().unwrap(),
                    test_file.to_str().unwrap(),
                    &format!("-l{}", lib_name),
                ])
                .status()
                .map(|status| status.success())
                .unwrap_or(false);

            if compile_result {
                return true;
            }

            // Fallback to checking common library locations
            let lib_paths = if arch == "aarch64" {
                vec!["/opt/homebrew/lib", "/usr/local/lib", "/usr/lib"]
            } else {
                vec!["/usr/local/lib", "/opt/homebrew/lib", "/usr/lib"]
            };

            for path in &lib_paths {
                let dylib = format!("{}/lib{}.dylib", path, lib_name);
                let a_file = format!("{}/lib{}.a", path, lib_name);

                if Path::new(&dylib).exists() || Path::new(&a_file).exists() {
                    return true;
                }
            }

            false
        }
        _ => {
            eprintln!("Unsupported operating system: {}", os);
            false
        }
    }
}
