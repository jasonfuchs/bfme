// Copyright © 2025 Jason Fuchs
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut file = env::args()
        .skip(1)
        .next()
        .map(|arg| if arg == "-" { "/dev/stdin".into() } else { arg })
        .map(|path| File::open(path))
        .ok_or("No input file specified")
        .map_err(io::Error::other)??;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;

    let mut data: Vec<u8> = vec![0u8; 30_000];
    let mut di: usize = 0;

    let mut bi = 0;
    while bi < buf.len() {
        match buf[bi] {
            b'<' => di -= 1,
            b'>' => di += 1,
            b'+' => data[di] = data[di].wrapping_add(1),
            b'-' => data[di] = data[di].wrapping_sub(1),
            b'.' => io::stdout().write_all(&data[di..di + 1])?,
            b',' => io::stdin().read_exact(&mut data[di..di + 1])?,
            b'[' if data[di] == 0 => {
                let mut deepth = 1;
                while deepth > 0 {
                    bi += 1;
                    match buf[bi] {
                        b'[' => deepth += 1,
                        b']' => deepth -= 1,
                        _ => (),
                    }
                }
            }
            b']' if data[di] != 0 => {
                let mut deepth = 1;
                while deepth > 0 {
                    bi -= 1;
                    match buf[bi] {
                        b'[' => deepth -= 1,
                        b']' => deepth += 1,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        bi += 1;
    }

    Ok(())
}
