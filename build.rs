fn main() -> std::io::Result<()> {
    if std::env::var("DOCS_RS").is_err() {
        #[allow(unused_mut)]
        let mut tonic_build_conf = tonic_build::configure().out_dir("src/"); // saves generated structures at this location

        #[cfg(feature = "serde")]
        {
            tonic_build_conf = tonic_build_conf
                .compile_well_known_types(true)
                .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
                .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp");
        }

        return tonic_build_conf.compile(
            &["proto/qdrant.proto"], // proto entry point
            &["proto"],              // specify the root location to search proto dependencies
        );
    }
    Ok(())
}
