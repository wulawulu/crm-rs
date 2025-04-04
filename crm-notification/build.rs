use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/notification/messages.proto",
                "../protos/notification/rpc.proto",
            ],
            &["../protos"],
        )
        .unwrap();

    Ok(())
}
