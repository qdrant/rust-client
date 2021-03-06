fn main() -> std::io::Result<()> {
    if std::env::var("DOCS_RS").is_err() {
        return tonic_build::configure()
            .out_dir("src/") // saves generated structures at this location
            .compile(
                &["proto/qdrant.proto"], // proto entry point
                &["proto"],              // specify the root location to search proto dependencies
            );
    }
    Ok(())
}
