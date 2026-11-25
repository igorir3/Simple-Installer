from pathlib import Path
import os
import hashlib
import tarfile
import subprocess
import zstandard as zstd
import shutil
import struct

env = os.environ.copy()
env["APP_NAME"] = input("Name of program: ")

while True:
    SOURCE = Path(input("Full name of program's folder: "))
    if SOURCE.exists():
        break
    print(f"{SOURCE} can't be find")

ASSETS = Path("assets")
ASSETS.mkdir(parents=True, exist_ok=True)

archive_path = ASSETS / "payload.tar.zst"
manifest_path = ASSETS / "manifest.txt"

files = sorted(
    p for p in SOURCE.rglob("*")
    if p.is_file() and not p.is_symlink()
)

with manifest_path.open("w", encoding="utf-8", newline="\n") as manifest:
    with archive_path.open("wb") as output:
        compressor = zstd.ZstdCompressor(level=12)

        with compressor.stream_writer(output) as compressed:
            with tarfile.open(fileobj=compressed, mode="w|") as archive:
                for file in files:
                    relative = file.relative_to(SOURCE).as_posix()

                    hasher = hashlib.sha256()
                    with file.open("rb") as source:
                        for chunk in iter(lambda: source.read(65536), b""):
                            hasher.update(chunk)

                    manifest.write(f"{hasher.hexdigest()}\t{relative}\n")

                    archive.add(file, arcname=relative, recursive=False)


subprocess.run(
    ["cargo", "build", "--release"],
    cwd=r"D:\Rust\Simple-Installer",
    env=env,
    check=True
)

exe = Path("target/release/Simple-Installer.exe")
output = Path("setup.exe")

shutil.copyfile(exe, output)

with output.open("ab") as dst:
    with archive_path.open("rb") as src:
        shutil.copyfileobj(src, dst, length=1024 * 1024)

    manifest_data = manifest_path.read_bytes()
    dst.write(manifest_data)

    dst.write(struct.pack(
        "<QQ8s",
        archive_path.stat().st_size,
        len(manifest_data),
        b"RUSTINST"
    ))