"""Dependency-free, deterministic PNG fixtures; originals are never overwritten."""
import pathlib
import struct
import sys
import zlib
root = pathlib.Path(sys.argv[1])
def chunk(kind, data):
    return struct.pack('!I', len(data)) + kind + data + struct.pack('!I', zlib.crc32(kind + data) & 0xffffffff)
for i, name in enumerate(('fern', 'moss', 'clover')):
    w, h = 1024, 768
    data = b''.join(b'\x00' + bytes(v for x in range(w) for v in ((x // 8 + i * 40) % 256, (y // 4 + 70) % 256, (x // 16 + y // 16 + 50) % 256)) for y in range(h))
    png = b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('!IIBBBBB', w, h, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(data)) + chunk(b'IEND', b'')
    (root / (name + '.png')).write_bytes(png)
