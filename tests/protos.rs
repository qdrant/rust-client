fn timestamp(f: impl AsRef<std::path::Path>) -> std::time::SystemTime {
    std::fs::metadata(f).unwrap().modified().unwrap()
}

#[test]
fn protos() {
    let out_time = timestamp("src/qdrant.rs");
    let mut protos = std::fs::read_dir("proto").unwrap();
    if protos.any(|d| timestamp(&d.unwrap().path()) > out_time) {
        tonic_build::configure()
            .out_dir("src/") // saves generated structures at this location
            .compile(
                &["proto/qdrant.proto"], // proto entry point
                &["proto"],              // specify the root location to search proto dependencies
            )
            .unwrap();
        panic!("proto definitions changed. Stubs recompiled. Please commit the changes.")
    }
}
