#!/usr/bin/env bash
# Syncs proto/serverless_collections.proto from qdrant-cloud-public-api and
# regenerates src/serverless/grpc.rs.
#
# Usage: ./tools/sync_serverless_proto.sh

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

HEADER='// Source: https://github.com/qdrant/qdrant-cloud-public-api/blob/main/proto/qdrant/serverless/collections.proto
// Renamed to serverless_collections.proto to sit alongside the regular collections.proto.
// Client copy: buf.validate options are stripped (server-side only; wire format unchanged).
// Regenerate with: cargo test --test serverless_protos -- --ignored --nocapture
'

PROTO_PATH="proto/serverless_collections.proto"
TMP_PROTO="$(mktemp)"
curl -fsSL https://raw.githubusercontent.com/qdrant/qdrant-cloud-public-api/main/proto/qdrant/serverless/collections.proto \
  > "$TMP_PROTO"

# Drop server-side protovalidate import/options; clients do not need them and
# vendoring buf/validate would pull an extra dependency into the sync path.
python3 - "$TMP_PROTO" "$PROTO_PATH" "$HEADER" <<'PY'
import re, sys
src_path, out_path, header = sys.argv[1], sys.argv[2], sys.argv[3]
src = open(src_path).read()
src = re.sub(r'\nimport "buf/validate/validate\.proto";\n', '\n', src)
src = re.sub(
    r' \[\(buf\.validate\.field\)\.uint32 = \{\s*gt: 0\s*lte: 100\s*\}\]',
    '',
    src,
)
if 'buf.validate' in src:
    raise SystemExit('failed to strip buf.validate annotations from collections.proto')
open(out_path, 'w').write(header + '\n' + src)
PY
rm -f "$TMP_PROTO"

cargo test --test serverless_protos regenerate_serverless_protos -- --ignored --nocapture

echo "Synced $PROTO_PATH and regenerated src/serverless/grpc.rs"
