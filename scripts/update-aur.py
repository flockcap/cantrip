#!/usr/bin/env python3
"""Pin AUR archive hashes using locally downloaded release archives."""
import hashlib
import pathlib
import re
import sys
folder = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "target/distrib")
path = pathlib.Path("packaging/aur/PKGBUILD")
text = path.read_text()
for arch in ("x86_64", "aarch64"):
    archive = folder / f"cantrip-cli-{arch}-unknown-linux-musl.tar.xz"
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    text = re.sub(rf"sha256sums_{arch}=\('[^']*'\)", f"sha256sums_{arch}=('{digest}')", text)
path.write_text(text)
