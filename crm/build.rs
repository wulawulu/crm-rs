use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    tonic_build::configure()
        .out_dir("src/pb")
        .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        .with_field_attributes(
            &["WelcomeRequest.content_ids", "RecallRequest.content_ids"],
            &[r#"#[builder(setter(each(name="content_id",into)))]"#],
        )
        .compile_protos(
            &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )?;

    Ok(())
}
