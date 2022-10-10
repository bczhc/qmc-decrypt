use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::io::{stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Arg, Command};

use qmc_decrypt::{qmcflac, AnyResult, Format};
use qmc_decrypt::{read_qmc_tag, TagName};

fn main() -> AnyResult<()> {
    let matches = Command::new("qmc-decrypt")
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("output").required(true))
        .arg(Arg::new("ekey").required(false))
        .get_matches();

    let input_path: PathBuf = matches.get_one::<String>("input").unwrap().into();
    let mut output_path: PathBuf = matches.get_one::<String>("output").unwrap().into();

    let extension = input_path.extension().and_then(|x| {
        let extension = x.to_str().expect("Invalid UTF-8");
        Format::from_str(extension).ok()
    });
    let format = extension.ok_or("Cannot recognize input file format")?;

    if output_path.is_dir() {
        let new_name = input_path
            .file_stem()
            .map(|x| {
                let mut new_name = OsString::from(x);
                new_name.push(".");
                new_name.push(format.decrypted_extension());
                new_name
            })
            .ok_or("Invalid input file name")?;
        output_path.push(&new_name);
    }

    eprint!("Decrypting {:?}... ", input_path);
    stdout().flush()?;
    match format {
        Format::QmcFlac | Format::Qmc0 => decrypt_qmcflac(input_path, output_path),
        Format::MFlac0 | Format::Mgg1 => {
            let ekey = matches.get_one::<String>("ekey");
            if ekey.is_none() {
                return Err("EKey is needed to decrypt files with STag".into());
            }
            let ekey = ekey.unwrap();

            decrypt_mflac0(input_path, output_path, ekey)
        }
    }?;
    eprintln!("done");
    Ok(())
}

fn decrypt_qmcflac<P: AsRef<Path>>(input: P, output: P) -> AnyResult<()> {
    let input = File::open(input)?;
    let mut output = open_output_file(output)?;
    let mut stream = qmcflac::read::Stream::new(input);
    io::copy(&mut stream, &mut output)?;
    Ok(())
}

fn decrypt_mflac0<P: AsRef<Path>>(input: P, output: P, ekey: &str) -> AnyResult<()> {
    let tag = read_qmc_tag(&input)?;
    match tag {
        Some(tag) if tag != TagName::STag => {
            return Err("Only support STag".into());
        }
        _ => {}
    }

    let qmc_crypto = qmc2_crypto::decrypt_factory(ekey).map_err(qmc_decrypt::CryptoError::from)?;

    let mut input = File::open(input)?;
    let mut output = open_output_file(output)?;
    let mut buf = [0_u8; 4096];
    let mut offset = 0_u64;
    loop {
        let size = input.read(&mut buf)?;
        if size == 0 {
            break;
        }

        qmc_crypto.decrypt(offset as usize, &mut buf);
        output.write_all(&buf)?;
        offset += size as u64;
    }
    Ok(())
}

fn open_output_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
}
