use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new().compile_protos(
        &[
            "src/proto/google/protobuf/descriptor.proto",
            "src/proto/deadlock/citadel_gcmessages_common.proto",
            "src/proto/deadlock/gcsdk_gcmessages.proto",
            "src/proto/deadlock/steammessages.proto",
            "src/proto/deadlock/steammessages_steamlearn.steamworkssdk.proto",
            "src/proto/deadlock/steammessages_unified_base.steamworkssdk.proto",
        ],
        &["src"],
    )?;
    Ok(())
}
