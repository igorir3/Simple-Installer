use std::{
    env,
    fs::File,
    io::{self, Read, Seek, SeekFrom, Take},
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub enum InstallEvent {
    TotalSize(usize),
    Progress(usize),
    Finished,
    Error(String),
}

fn get_payload() -> io::Result<(Take<File>, String)> {
    let mut file = File::open(env::current_exe()?)?;
    let file_size = file.metadata()?.len();

    file.seek(SeekFrom::End(-24))?;

    let mut footer = [0u8; 24];
    file.read_exact(&mut footer)?;

    if &footer[16..24] != b"RUSTINST" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid installer signature",
        ));
    }

    let archive_size =
        u64::from_le_bytes(footer[0..8].try_into().unwrap());

    let manifest_size =
        u64::from_le_bytes(footer[8..16].try_into().unwrap());

    let archive_offset = file_size
        .checked_sub(24)
        .and_then(|n| n.checked_sub(manifest_size))
        .and_then(|n| n.checked_sub(archive_size))
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid payload size",
        ))?;

    file.seek(SeekFrom::Start(archive_offset + archive_size))?;

    let mut manifest = String::new();

    (&mut file)
        .take(manifest_size)
        .read_to_string(&mut manifest)?;

    file.seek(SeekFrom::Start(archive_offset))?;

    Ok((file.take(archive_size), manifest))
}

pub fn start_installation(path: PathBuf) -> Receiver<InstallEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let event = match unpack(path, &tx) {
            Ok(()) => InstallEvent::Finished,
            Err(e) => InstallEvent::Error(e.to_string()),
        };

        let _ = tx.send(event);
    });

    rx
}

fn unpack(
    path: PathBuf,
    tx: &Sender<InstallEvent>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&path)?;

    let (archive, manifest) = get_payload()?;
    let decoder = zstd::Decoder::new(archive)?;
    let mut archive = tar::Archive::new(decoder);


    let _ = tx.send(InstallEvent::TotalSize(manifest.lines().count()));

    let mut count = 0;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let is_file = entry.header().entry_type().is_file();

        if !entry.unpack_in(&path)? {
            return Err(std::io::Error::other("Недопустимый путь"));
        }

        if is_file {
            count += 1;
            let _ = tx.send(InstallEvent::Progress(count));
        }
    }

    for line in manifest.lines() {
        if let Some((hash, path)) = line.split_once('|') {
            println!("{path}: {hash}");
        }
    }

    Ok(())
}