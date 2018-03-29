include!(concat!(env!("OUT_DIR"), "/svd_bits.rs"));

extern crate tempdir;
use tempdir::TempDir;

use std::env;
use std::io;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

fn extract_all_frags(dir: &Path) -> io::Result<()> {
    for &(frag_bytes, frag_fn) in ALL_FRAGS {
        let filename = dir.join(frag_fn);
        let mut f = File::create(filename)?;
        f.write_all(frag_bytes)?;
    }

    Ok(())
}

fn match_dev_name(devname: &str) -> Option<(&'static [u8], &'static str, &'static str)> {
    match devname.to_ascii_lowercase().as_ref() {
        "efm32hg108f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG108", "FLASH_32")),
        "efm32hg108f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG108", "FLASH_64")),
        "efm32hg110f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG110", "FLASH_32")),
        "efm32hg110f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG110", "FLASH_64")),
        "efm32hg210f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG210", "FLASH_32")),
        "efm32hg210f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG210", "FLASH_64")),
        "efm32hg222f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG222", "FLASH_32")),
        "efm32hg222f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG222", "FLASH_64")),
        "efm32hg308f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG308", "FLASH_32")),
        "efm32hg308f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG308", "FLASH_64")),
        "efm32hg309f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG309", "FLASH_32")),
        "efm32hg309f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG309", "FLASH_64")),
        "efm32hg310f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG310", "FLASH_32")),
        "efm32hg310f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG310", "FLASH_64")),
        "efm32hg321f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG321", "FLASH_32")),
        "efm32hg321f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG321", "FLASH_64")),
        "efm32hg322f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG322", "FLASH_32")),
        "efm32hg322f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG322", "FLASH_64")),
        "efm32hg350f32" => Some((EFM32HG_TMPL_SVD, "EFM32HG350", "FLASH_32")),
        "efm32hg350f64" => Some((EFM32HG_TMPL_SVD, "EFM32HG350", "FLASH_64")),
        _ => None,
    }
}

fn main() {
    let argv = env::args().collect::<Vec<_>>();

    if argv.len() != 2 {
        println!("Usage: {} device-name", argv[0]);
        std::process::exit(1);
    }

    let tmpl_info = match_dev_name(&argv[1]);
    if tmpl_info.is_none() {
        println!("Unsupported device");
        std::process::exit(1);
    }
    let (tmpl_bytes, tmpl_def, tmpl_flash_def) = tmpl_info.unwrap();

    let dir = TempDir::new("efm32rs").unwrap();
    extract_all_frags(dir.path()).unwrap();

    let svd_in_filename = dir.path().join("svd.tmpl");
    {
        let mut f = File::create(&svd_in_filename).unwrap();
        f.write_all(tmpl_bytes).unwrap();
    }

    // Run the preprocess step
    let svd_out_filename = dir.path().join("svd.svd");
    let preprocess_result = Command::new("gcc")
        .arg("-x").arg("c")
        .arg("-E")
        .arg("-P")
        .arg(format!("-D{}", tmpl_def))
        .arg(format!("-D{}", tmpl_flash_def))
        .arg("-o").arg(&svd_out_filename)
        .arg(svd_in_filename)
        .status().unwrap();
    if !preprocess_result.success() {
        println!("Preprocessing failed");
        std::process::exit(1);
    }

    // Run svd2rust
    let svd2rust_output = Command::new("svd2rust")
        .arg("-i").arg(svd_out_filename)
        .output().unwrap();
    if !svd2rust_output.status.success() {
        println!("svd2rust failed");
        std::process::exit(1);
    }

    // Run rustfmt
    let mut rustfmt_child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .spawn().unwrap();

    {
        let stdin = rustfmt_child.stdin.as_mut().unwrap();
        stdin.write_all(&svd2rust_output.stdout).unwrap();
    }
    let rustfmt_output = rustfmt_child.wait_with_output().unwrap();
    if !(rustfmt_output.status.success() ||
        (rustfmt_output.status.code().is_some() && rustfmt_output.status.code().unwrap() == 3)) {
        println!("rustfmt failed");
        std::process::exit(1);
    }

    io::stdout().write(&rustfmt_output.stdout).unwrap();
}
