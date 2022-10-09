use std::fs::File;
use std::io;
use std::io::{stdout, Read, Write};
use std::path::Path;

use clap::{Arg, Command};

use qmc_decrypt::qmcflac;
use qmc_decrypt::{read_qmc_tag, TagName};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("qmc-decrypt")
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("output").required(true))
        .arg(Arg::new("ekey").required(false))
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    let tag = read_qmc_tag(input_path)?;

    match tag {
        Some(tag) if tag != TagName::STag => {
            return Err("Only support STag".into());
        }
        _ => {}
    }

    eprint!("Decrypting {:?}... ", Path::new(input_path));
    stdout().flush()?;

    if Path::new(input_path)
        .extension()
        .map(|x| x.eq_ignore_ascii_case("qmcflac"))
        .unwrap_or(false)
    {
        let output = open_output_file(output_path)?;
        let mut stream = qmcflac::Stream::new(output);
        let mut input = File::open(input_path)?;
        io::copy(&mut input, &mut stream)?;
    } else {
        let ekey = matches.get_one::<String>("ekey");
        if ekey.is_none() {
            return Err("Missing EKey to decrypt files with STag".into());
        }
        let ekey = ekey.unwrap();

        decode_qmc_with_ekey(input_path, output_path, ekey)?;
    }

    eprintln!("done");

    Ok(())
}

fn decode_qmc_with_ekey<P: AsRef<Path>>(
    input: P,
    output: P,
    ekey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
