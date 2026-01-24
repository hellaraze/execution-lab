use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn find_last_good_offset(path: impl AsRef<Path>) -> Result<(u64, u64)> {
    let file = OpenOptions::new()
        .read(true)
        .open(path.as_ref())
        .with_context(|| format!("open {:?}", path.as_ref()))?;
    let mut reader = BufReader::new(file);

    let mut offset: u64 = 0;
    let mut last_good_offset: u64 = 0;
    let mut last_seq: u64 = 0;

    let mut line = String::new();
    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim_end();
        match serde_json::from_str::<eventlog::envelope::EventEnvelope>(trimmed) {
            Ok(env) => {
                last_good_offset = offset + (bytes as u64);
                last_seq = env.seq;
            }
            Err(_) => {
                // bad line -> stop, everything after is tail garbage
                break;
            }
        }

        offset += bytes as u64;
    }

    Ok((last_good_offset, last_seq))
}

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/el_test.log".to_string());

    let (good_off, last_seq) = find_last_good_offset(&path)?;
    let mut f = OpenOptions::new()
        .write(true)
        .open(&path)
        .with_context(|| format!("open for write {:?}", path))?;

    let len = f.metadata()?.len();
    if good_off < len {
        eprintln!("TRUNCATE: {} -> {} (last_seq={})", len, good_off, last_seq);
        f.set_len(good_off).context("set_len")?;
        f.seek(SeekFrom::End(0)).ok();
    } else {
        eprintln!(
            "OK: no truncate needed (len={}, last_seq={})",
            len, last_seq
        );
    }

    Ok(())
}
