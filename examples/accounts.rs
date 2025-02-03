use kilnfi_connect_rs::Kiln;
// required for uuid
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token: String = std::env::var("KILN_API_TOKEN")
        .expect("KILN_API_TOKEN is not set in the environment. It is required.");

    let kiln: Kiln = Kiln::builder()
        .api_token(api_token)
        // no trailing slash, or else it will break
        .base_url("https://api.kiln.fi/v1")
        .seal()
        .build()?;

    let uuid = uuid::Uuid::from_str("9e1a6784-9657-4757-b463-8454a34c92b4").unwrap();

    let _data = kiln.accounts().get_by_uuid(uuid);

    Ok(())
}
