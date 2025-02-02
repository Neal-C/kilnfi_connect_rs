// required for uuid
use kilnfi_connect_rs::Kiln;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token: String = std::env::var("API_TOKEN")
        .expect("API_TOKEN is not set in the environment. It is required.");

    let kiln: Kiln = Kiln::builder()
        .api_token(api_token)
        // no trailing slash, or else it will break
        .base_url(String::from("https://api.kiln.fi/v1"))
        .seal()
        .build()
        .unwrap();

    let _data = kiln.defi().network_stats();

    Ok(())
}
