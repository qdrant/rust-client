fn main() -> std::io::Result<()> {
    if std::env::var("DOCS_RS").is_err() {
        return tonic_build::configure()
            .compile_well_known_types(true)
            .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
            .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
            .out_dir("src/") // saves generated structures at this location
            .compile(
                &["proto/qdrant.proto"], // proto entry point
                &["proto"],              // specify the root location to search proto dependencies
            );
    }
    Ok(())
}
