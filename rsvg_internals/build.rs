use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[cfg(all(unix, feature = "c-library"))]
use std::os::unix::fs::symlink;

#[cfg(all(windows, not(target_env = "msvc"), feature="c-library"))]
use std::os::windows::fs::symlink_file as symlink;

#[cfg(all(not(target_env = "msvc"), feature="c-library"))]
use std::fs;
#[cfg(all(not(target_env = "msvc"), feature="c-library"))]
use std::path::PathBuf;

fn main() {
    generate_srgb_tables();

    generate_convenience_lib().unwrap();
}

/// Converts an sRGB color value to a linear sRGB color value (undoes the gamma correction).
///
/// The input and the output are supposed to be in the [0, 1] range.
#[inline]
fn linearize(c: f64) -> f64 {
    if c <= (12.92 * 0.0031308) {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts a linear sRGB color value to a normal sRGB color value (applies the gamma correction).
///
/// The input and the output are supposed to be in the [0, 1] range.
#[inline]
fn unlinearize(c: f64) -> f64 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1f64 / 2.4) - 0.055
    }
}

fn compute_table<F: Fn(f64) -> f64>(f: F) -> [u8; 256] {
    let mut table = [0; 256];

    for i in 0..=255 {
        let c = i as f64 / 255.0;
        let x = f(c);
        table[i] = (x * 255.0).round() as u8;
    }

    table
}

fn print_table<W: Write>(w: &mut W, name: &str, table: &[u8]) {
    writeln!(w, "const {}: [u8; {}] = [", name, table.len()).unwrap();

    for x in table {
        writeln!(w, "    {},", x).unwrap();
    }

    writeln!(w, "];").unwrap();
}

fn generate_srgb_tables() {
    let linearize_table = compute_table(linearize);
    let unlinearize_table = compute_table(unlinearize);

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("srgb-codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    print_table(&mut file, "LINEARIZE", &linearize_table);
    print_table(&mut file, "UNLINEARIZE", &unlinearize_table);
}

/// Generate libtool archive file librsvg_internals.la
/// From: https://docs.rs/libtool/0.1.1/libtool/
#[cfg(all(feature = "c-library", not(target_env = "msvc")))]
pub fn generate_convenience_lib() -> std::io::Result<()> {
    let target = env::var("TARGET").expect("TARGET was not set");
    let build_dir = env::var("LIBRSVG_BUILD_DIR").expect("LIBRSVG_BUILD_DIR was not set");
    let target_dir = env::var("LIBRSVG_TARGET_DIR").expect("LIBRSVG_TARGET_DIR was not set");
    let libs_dir = format!("{}/.libs", build_dir);
    let libs_path = PathBuf::from(&libs_dir);
    let la_path = PathBuf::from(format!("{}/librsvg_internals.la", build_dir));
    let rust_lib = if target.contains("windows") {
        /* https://github.com/rust-lang/rust/issues/43749 */
        "rsvg_internals.lib"
    } else {
        "librsvg_internals.a"
    };
    let old_lib_path = PathBuf::from(format!("{}/{}", target_dir, rust_lib));
    let new_lib_path = PathBuf::from(format!("{}/librsvg_internals.a", libs_dir));

    match fs::create_dir_all(&libs_path) {
        Ok(()) => println!("libs_path created"),
        _ => panic!("Failed to create libs_path"),
    }

    if la_path.exists() {
        fs::remove_file(&la_path)?;
    }

    /* PathBuf.exists() traverses symlinks so just try and remove it */
    match fs::remove_file(&new_lib_path) {
        Ok(_v) => {},
        Err(e) => println!("Error removing symlink: {:?}", e),
    }

    let mut file = File::create(&la_path).unwrap();
    writeln!(file, "# librsvg_internals.la - a libtool library file")?;
    writeln!(file, "# Generated by libtool-rust")?;
    writeln!(file, "dlname=''")?;
    writeln!(file, "library_names=''")?;
    writeln!(file, "old_library='librsvg_internals.a'")?;
    writeln!(file, "inherited_linker_flags=''")?;
    writeln!(file, "installed=no")?;
    writeln!(file, "shouldnotlink=no")?;
    symlink(&old_lib_path, &new_lib_path)?;
    Ok(())
}

#[cfg(not(all(feature = "c-library", not(target_env = "msvc"))))]
pub fn generate_convenience_lib() -> std::io::Result<()> {
    Ok(())
}
