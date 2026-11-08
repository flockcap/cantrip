#!/bin/sh
set -eu
dist generate
python3 - <<'PY'
from pathlib import Path
p = Path('.github/workflows/release.yml')
text = p.read_text().replace('uses: $/.github/workflows/', 'uses: ./.github/workflows/')
text = text.replace('      \"id-token\": \"write\"\n      \"packages\": \"write\"', '      \"contents\": \"read\"')
p.write_text(text)
PY
