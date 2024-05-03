use tonic_build::Builder;

fn timestamp(f: impl AsRef<std::path::Path>) -> std::time::SystemTime {
    std::fs::metadata(f).unwrap().modified().unwrap()
}

const GRPC_OUTPUT_FILE: &str = "src/qdrant.rs";

#[test]
fn protos() {
    let out_time = timestamp(GRPC_OUTPUT_FILE);
    let mut protos = std::fs::read_dir("proto").unwrap();
    if !protos.any(|d| timestamp(d.unwrap().path()) > out_time)
        && timestamp("tests/protos.rs") <= out_time
    {
        return;
    }

    tonic_build::configure()
        .configure_derive_builder()
        .out_dir("src/") // saves generated structures at this location
        .compile(
            &["proto/qdrant.proto"], // proto entry point
            &["proto"],              // specify the root location to search proto dependencies
        )
        .unwrap();

    // Append macro definition and calls required for builder implementations.
    append_file_to_file(GRPC_OUTPUT_FILE, "./tests/protos_append/grpc_macros.rs");
    add_builder_macro_impls(GRPC_OUTPUT_FILE, builder_derive_options());

    append_file_to_file(GRPC_OUTPUT_FILE, "./tests/protos_append/builder_ext.rs");

    panic!("proto definitions changed. Stubs recompiled. Please commit the changes.")
}

/// Derive options for structs. (Path, build attributes, 'from' macro generation enabled)
type BuildDeriveOptions = (&'static str, &'static str, bool);

/// Extension to [`Builder`] to configure builder attributes.
trait BuilderExt {
    fn configure_derive_builder(self) -> Self;
    fn derive_builders(self, paths: &[(&str, &str)], derive_options: &[BuildDeriveOptions])
        -> Self;
    fn derive_builder(self, path: &str, derive_options: Option<&str>) -> Self;
    fn field_build_attributes(self, paths: &[(&str, &str)]) -> Self;
}

impl BuilderExt for Builder {
    fn configure_derive_builder(self) -> Self {
        configure_builder(self)
    }

    fn derive_builders(
        self,
        paths: &[(&str, &str)],
        derive_options: &[BuildDeriveOptions],
    ) -> Self {
        let structs = unique_structs_from_paths(paths.iter().map(|i| i.0), &[]);

        let derives = structs.into_iter().fold(self, |c, path| {
            let derive_options = derive_options.iter().find(|i| i.0 == path).map(|i| i.1);
            c.derive_builder(path, derive_options)
        });

        derives.field_build_attributes(paths)
    }

    fn derive_builder(self, path: &str, derive_options: Option<&str>) -> Self {
        let builder = self.type_attribute(path, "#[derive(derive_builder::Builder)]");

        if let Some(derive_options) = derive_options {
            builder.type_attribute(path, format!("#[builder({derive_options})]"))
        } else {
            builder
        }
    }

    fn field_build_attributes(self, paths: &[(&str, &str)]) -> Self {
        paths.iter().fold(self, |c, (path, attribute)| {
            c.field_attribute(path, format!("#[builder({attribute})]"))
        })
    }
}

/// Generates a str for field attributes required by some attributes for derive_builder.
/// This allows the builders fields being represented by different types than the actual built struct, if they can be converted.
/// This happens to be the case for quite some generated grpc types, so we use this to allow a simpler API for users.
///
/// The generated str looks like this:
/// setter(into, strip_option), field(ty="Option<$ty>", build="convert_option(&$id)")
///
/// For more infos of those atributes see the [dervie_builder's docs](https://docs.rs/derive_builder/latest/derive_builder/#completely-custom-fields-in-the-builder)
macro_rules! builder_custom_into {
    ($ty:ty, $id:expr) => {
        // Builds the following string as &'static str:
        // setter(into, strip_option), field(ty="Option<$ty>", build="convert_option(&$id)")
        concat!(
            "setter(into, strip_option)",
            ",",
            concat!("field(ty=\"", "Option<", stringify!($ty), ">\""),
            ",",
            concat!("build=\"", "convert_option(&", stringify!($id), ")\""),
            ")"
        )
    };
}

fn configure_builder(builder: Builder) -> Builder {
    const DEFAULT_OPTION: &str = "default, setter(strip_option)";
    const DEFAULT_OPTION_INTO: &str = "default, setter(into, strip_option)";
    const DEFAULT: &str = "default";
    const DEFAULT_INTO: &str = "default, setter(into)";

    builder.derive_builders(
        &[
            // VectorParams
            ("VectorParams.size", DEFAULT),
            ("VectorParams.distance", DEFAULT_INTO),
            ("VectorParams.hnsw_config", DEFAULT_OPTION_INTO),
            // ("VectorParams.quantization_config", DEFAULT_OPTION_INTO),
            (
                "VectorParams.quantization_config",
                builder_custom_into!(quantization_config::Quantization, self.quantization_config),
            ),
            ("VectorParams.on_disk", DEFAULT_OPTION),
            ("VectorParams.datatype", DEFAULT_OPTION_INTO),
            // Create collection
            ("CreateCollection.collection_name", DEFAULT_INTO),
            ("CreateCollection.hnsw_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.wal_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.optimizers_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.shard_number", DEFAULT_OPTION),
            ("CreateCollection.on_disk_payload", DEFAULT_OPTION),
            ("CreateCollection.timeout", DEFAULT_OPTION),
            (
                "CreateCollection.vectors_config",
                builder_custom_into!(vectors_config::Config, self.vectors_config),
                // => "setter(into, strip_option), field(ty=Option<vectors_config::Config>, build=convert_option(&self.vectors_config))"
            ),
            ("CreateCollection.replication_factor", DEFAULT_OPTION),
            ("CreateCollection.write_consistency_factor", DEFAULT_OPTION),
            ("CreateCollection.init_from_collection", DEFAULT_OPTION_INTO),
            // ("CreateCollection.quantization_config", DEFAULT_OPTION_INTO),  (
            (
                "CreateCollection.quantization_config",
                builder_custom_into!(quantization_config::Quantization, self.quantization_config),
            ),
            ("CreateCollection.sharding_method", DEFAULT_OPTION),
            (
                "CreateCollection.sparse_vectors_config",
                DEFAULT_OPTION_INTO,
            ),
            // HnswConfig
            ("HnswConfigDiff.m", DEFAULT_OPTION),
            ("HnswConfigDiff.ef_construct", DEFAULT_OPTION),
            ("HnswConfigDiff.full_scan_threshold", DEFAULT_OPTION),
            ("HnswConfigDiff.max_indexing_threads", DEFAULT_OPTION),
            ("HnswConfigDiff.on_disk", DEFAULT_OPTION),
            ("HnswConfigDiff.payload_m", DEFAULT_OPTION),
            // ScalarQuantization
            ("ScalarQuantization.quantile", DEFAULT_OPTION),
            ("ScalarQuantization.always_ram", DEFAULT_OPTION),
            // ProductQuantization
            ("ProductQuantization.always_ram", DEFAULT_OPTION),
            // BinaryQuantization
            ("BinaryQuantization.always_ram", DEFAULT_OPTION),
            // OptimizersConfigDiff
            ("OptimizersConfigDiff.deleted_threshold", DEFAULT_OPTION),
            (
                "OptimizersConfigDiff.vacuum_min_vector_number",
                DEFAULT_OPTION,
            ),
            (
                "OptimizersConfigDiff.default_segment_number",
                DEFAULT_OPTION,
            ),
            ("OptimizersConfigDiff.max_segment_size", DEFAULT_OPTION),
            ("OptimizersConfigDiff.memmap_threshold", DEFAULT_OPTION),
            ("OptimizersConfigDiff.indexing_threshold", DEFAULT_OPTION),
            ("OptimizersConfigDiff.flush_interval_sec", DEFAULT_OPTION),
            (
                "OptimizersConfigDiff.max_optimization_threads",
                DEFAULT_OPTION,
            ),
            //WalConfigDiff
            ("WalConfigDiff.wal_capacity_mb", DEFAULT_OPTION),
            ("WalConfigDiff.wal_segments_ahead", DEFAULT_OPTION),
            // SearchPoints
            ("SearchPoints.filter", DEFAULT_OPTION_INTO),
            (
                "SearchPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("SearchPoints.params", DEFAULT_OPTION_INTO),
            ("SearchPoints.score_threshold", DEFAULT_OPTION),
            ("SearchPoints.offset", DEFAULT_OPTION),
            ("SearchPoints.vector_name", DEFAULT_OPTION_INTO),
            (
                "SearchPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "SearchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("SearchPoints.timeout", DEFAULT_OPTION),
            ("SearchPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            ("SearchPoints.sparse_indices", DEFAULT_OPTION_INTO),
            // SearchParams
            ("SearchParams.hnsw_ef", DEFAULT_OPTION),
            ("SearchParams.exact", DEFAULT_OPTION),
            ("SearchParams.quantization", DEFAULT_OPTION_INTO),
            ("SearchParams.indexed_only", DEFAULT_OPTION),
            // QuantizationSearchParams
            ("QuantizationSearchParams.ignore", DEFAULT_OPTION),
            ("QuantizationSearchParams.rescore", DEFAULT_OPTION),
            ("QuantizationSearchParams.oversampling", DEFAULT_OPTION),
        ],
        builder_derive_options(),
    )
}

/// Builder configurations for grpc structs.
fn builder_derive_options() -> &'static [BuildDeriveOptions] {
    // We populate our own .build() function that doesn't return a Result but the built type directly.
    // For this we rename and make the automatically generated build function private.
    // Infallible allows secure unwrapping and compiler errors on missing fields.
    const DEFAULT_BUILDER_DERIVE_OPTIONS: &str =
        "build_fn(private, error = \"std::convert::Infallible\", name = \"build_inner\")";
    const NO_DEFAULT_BUILDER_DERIVE_OPTIONS: &str =
        "build_fn(private, name = \"build_inner\"), custom_constructor";

    // Tuple structure: (Path, build attributes, 'from' macro generation enabled)
    &[
        ("CreateCollection", DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        ("VectorParams", NO_DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        ("HnswConfigDiff", DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        (
            "ScalarQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            true,
        ),
        (
            "ProductQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            true,
        ),
        (
            "BinaryQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            true,
        ),
        ("OptimizersConfigDiff", DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        ("WalConfigDiff", DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        ("SearchPoints", NO_DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        ("SearchParams", DEFAULT_BUILDER_DERIVE_OPTIONS, true),
        (
            "QuantizationSearchParams",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            true,
        ),
    ]
}

/// Returns a list of all unique structs that appear in a list of paths.
fn unique_structs_from_paths<'a, I>(paths: I, extra: &[&'a str]) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut derives = paths
        .into_iter()
        .map(|field| field.split_once('.').unwrap().0)
        .collect::<Vec<&str>>();
    derives.extend(extra);
    derives.sort_unstable();
    derives.dedup();
    derives
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

fn append_file_to_file(output: &str, input: &str) {
    let src = std::fs::read_to_string(input).unwrap();
    append_to_file(output, &src);
}

/// Generates all necessary macro calls for builders who should have them.
fn add_builder_macro_impls(file: &str, derive_options: &[BuildDeriveOptions]) {
    let to_append = derive_options
        .iter()
        .filter_map(|i| i.2.then_some(i.0))
        .map(|i| format!("builder_type_conversions!({i}, {i}Builder);\n"))
        .fold(String::new(), |mut s, line| {
            s.push_str(&line);
            s
        });
    append_to_file(file, &to_append);
}
