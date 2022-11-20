use qmc2_crypto::detection::RECOMMENDED_DETECTION_SIZE;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const NEG_DETECTION_POS: i64 = -(RECOMMENDED_DETECTION_SIZE as i64);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    eprintln!("QMC2-decoder (rust) v0.0.6 by Jixun");
    eprintln!("Licensed under the MIT License & Apache License 2.0.");
    eprintln!();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} 'encrypted_input_path' 'decrypted_output_path'",
            args[0]
        );
        eprintln!();

        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let mut detection_buf = vec![0u8; RECOMMENDED_DETECTION_SIZE];

    let mut input_file = File::open(&input_path).unwrap();

    input_file.seek(SeekFrom::End(NEG_DETECTION_POS)).unwrap();
    input_file.read_exact(&mut detection_buf).unwrap();
    let detection = qmc2_crypto::detection::detect(&detection_buf).unwrap();

    eprint!("song id: ");
    if detection.song_id.is_empty() {
        eprintln!("(not found)")
    } else {
        eprintln!("{}", detection.song_id);
    };

    input_file
        .seek(SeekFrom::End(NEG_DETECTION_POS + detection.ekey_position))
        .unwrap();

    let mut ekey_buf = vec![0u8; detection.ekey_len];
    input_file.read_exact(&mut ekey_buf).unwrap();
    let ekey = std::str::from_utf8(&ekey_buf).unwrap();

    let decryptor = qmc2_crypto::decrypt_factory(ekey).expect("Could not extract ekey");

    let mut output_file = File::create(&output_path).unwrap();
    input_file
        .seek(SeekFrom::End(NEG_DETECTION_POS + detection.eof_position))
        .unwrap();
    let mut bytes_to_decrypt = input_file.stream_position().unwrap() as usize;
    input_file.seek(SeekFrom::Start(0)).unwrap();

    let mut offset = 0usize;
    let mut buf = vec![0u8; decryptor.get_recommended_block_size()];

    eprint!("Decrypting..");
    while bytes_to_decrypt > 0 {
        // Don't read over the file...
        let expected_read_size = std::cmp::min(bytes_to_decrypt, buf.len());
        let read_size = input_file.read(&mut buf[0..expected_read_size]).unwrap();
        decryptor.decrypt(offset, &mut buf[0..read_size]);
        output_file.write_all(&buf[0..read_size]).unwrap();
        eprint!(".");

        // Keep track of the progress.
        bytes_to_decrypt -= read_size;
        offset += read_size;
    }

    input_file.try_clone().unwrap();
    output_file.try_clone().unwrap();

    eprintln!("done!");
}
