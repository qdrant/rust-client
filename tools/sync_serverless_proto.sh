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
// Regenerate with: cargo test --test serverless_protos -- --ignored --nocapture
'

PROTO_PATH="proto/serverless_collections.proto"
{
  echo "$HEADER"
  curl -fsSL https://raw.githubusercontent.com/qdrant/qdrant-cloud-public-api/main/proto/qdrant/serverless/collections.proto
} > "$PROTO_PATH"

cargo test --test serverless_protos regenerate_serverless_protos -- --ignored --nocapture

echo "Synced $PROTO_PATH and regenerated src/serverless/grpc.rs"
