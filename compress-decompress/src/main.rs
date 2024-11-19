extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn handle_compression() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`",);
        return;
    }
    let mut input: BufReader<File> = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output: File = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder: GzEncoder<File> = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    let duration = start.elapsed();
    println!(
        "Compressed: {:?} ",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target: {:?} ", output.metadata().unwrap());
    println!("Duration: {} ms", duration.as_millis());
}
fn real_main() -> i32 {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = &*args[1];

    let input = File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(input).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.name();
        let mut outfile = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!(" File  (comment: {})", comment);
            }
        }
        if (*file.name()).ends_with('/') {
            println!(" File {} extracted  -> {}", file.name(), outfile.display());
            std::fs::create_dir_all(&outfile).unwrap();
        } else {
            println!(
                " File {} extracted  -> {} ({} bytes)",
                file.name(),
                outfile.display(),
                file.size()
            );

            if let Some(p) = outfile.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outfile).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outfile, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}

fn main() {
    std::process::exit(real_main());
}
