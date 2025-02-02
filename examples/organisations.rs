// required for uuid
use kilnfi_connect_rs::Kiln;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token: String = std::env::var("API_TOKEN")
        .expect("API_TOKEN is not set in the environment. It is required.");

    let kiln: Kiln = Kiln::builder()
        .api_token(api_token)
        // no trailing slash, or else it will break
        .base_url(String::from("https://api.kiln.fi/v1"))
        .seal()
        .build()?;

    let uuid = uuid::Uuid::from_str("9e1c62a4-2b01-4cd8-a2a4-74c57dcd2be4").unwrap();

    let _data = kiln.organisations().get_by_uuid(uuid);

    Ok(())
}
