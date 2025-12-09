use tonic_prost_build::Builder;

fn timestamp(f: impl AsRef<std::path::Path>) -> std::time::SystemTime {
    std::fs::metadata(f).unwrap().modified().unwrap()
}

const GRPC_OUTPUT_FILE: &str = "src/qdrant.rs";

#[test]
fn protos() {
    let out_time = timestamp(GRPC_OUTPUT_FILE);
    let mut protos = std::fs::read_dir("proto").unwrap();
    // Make sure the proto files are not dirty
    let protos_files_synced = !protos.any(|d| timestamp(d.unwrap().path()) > out_time);
    let protos_test_synced = timestamp("tests/protos.rs") <= out_time;
    if protos_files_synced && protos_test_synced {
        println!("protobuf files not changed. Exiting early!");
        return;
    }

    tonic_prost_build::configure()
        .configure_deprecations()
        .out_dir("src/") // saves generated structures at this location
        .compile_protos(
            &["proto/qdrant.proto"], // proto entry point
            &["proto"],              // specify the root location to search proto dependencies
        )
        .unwrap();

    // Re-export all custom builder here so they are all located in the same module in the end-user
    // API.
    let custom_reexports = [
        "pub use crate::manual_builder::*;",
        "pub use crate::builder_types::*;",
        "pub use crate::qdrant_client::builders::*;",
        "pub use crate::builders::*;",
    ];
    append_to_file(GRPC_OUTPUT_FILE, &custom_reexports.join("\n"));

    // Vendor gRPC types used in our objects
    append_to_file(GRPC_OUTPUT_FILE, "pub use prost_types::Timestamp;");

    eprintln!("protos_files_synced:{protos_files_synced}, protos_test_synced:{protos_test_synced}");
    eprintln!("proto definitions may be changed. Stubs recompiled. Please commit the changes.")
}

/// Extension to [`Builder`] to configure builder attributes.
trait BuilderExt {
    fn configure_deprecations(self) -> Self;
}

impl BuilderExt for Builder {
    fn configure_deprecations(self) -> Self {
        // Clear deprecated field for VectorOutput.data

        self.field_attribute(
            "PointsUpdateOperation.operation.delete_deprecated",
            "#[deprecated(since = \"1.7.0\", note = \"use `DeletePoints` instead\")]",
        )
        .field_attribute(
            "PointsUpdateOperation.operation.clear_payload_deprecated",
            "#[deprecated(since = \"1.7.0\", note = \"use `ClearPayload` instead\")]",
        )
        .field_attribute(
            "Vector.data",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`vector`](crate::qdrant::Vector::vector) field instead.\"]",
        )
        .field_attribute(
            "Vector.indices",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`vector`](crate::qdrant::Vector::vector) field instead.\"]",
        )
        .field_attribute(
            "Vector.vectors_count",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`vector`](crate::qdrant::Vector::vector) field instead.\"]",
        )
        .field_attribute(
            "VectorOutput.data",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`into_vector`](crate::qdrant::VectorOutput::into_vector) method instead.\"]",
        )
        .field_attribute(
            "VectorOutput.indices",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`into_vector`](crate::qdrant::VectorOutput::into_vector) method instead.\"]",
        )
        .field_attribute(
            "VectorOutput.vectors_count",
            "#[doc = \"\n\nDeprecated since 1.16.0, use [`into_vector`](crate::qdrant::VectorOutput::into_vector) method instead.\"]",
        )
    }
}

fn append_to_file(path: &str, line: &str) {
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    writeln!(
        OpenOptions::new().append(true).open(path).unwrap(),
        "{line}",
    )
    .unwrap()
}
