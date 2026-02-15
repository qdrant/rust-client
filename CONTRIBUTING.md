
# Upgrade for a new version

Here is a checklist for upgrading Rust Qdrant client to a new version of Qdrant server:

- [ ] Make sure to switch to a new branch from `dev`. Something like `v1-XX-upgrade` should be good enough.
- [ ] Synchronize protobuf definitions using `./tools/sync_proto.sh` script.
- [ ] Run `cargo test protos` to make sure auto-generated code is generated.

Based on the changes in protobuf, there are following places to upgrade:

- [ ] if there are new APIs, they should be added to appropriate file in `src/qdrant_client` and be part of `impl Qdrant`
- [ ] if there are new parameters for existing APIs, they should be added to appropriate builders in `src/qdrant_client/builders`
- [ ] if there are new structures, which require complicated construction, simplified versions should be added to `src/qdrant_client/conversions` like for example `impl From<&[f32]> for Vector`
- [ ] Examples with usage of all new changes should be added to `tests/snippets`, similar to existing ones.



