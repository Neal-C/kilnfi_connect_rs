> [!NOTE]
> follows current stable API version

## Description

Kiln connect API Rust client

## Installation

You can install the Rust SDK with cargo:

```shell
cargo add kilnfi_connect_rs
```

## Requirements

- a kiln api token. Please contact support@kiln.fi to get one.

## Example
```rs
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

    let uuid = uuid::Uuid::from_str("9e1a6784-9657-4757-b463-8454a34c92b4").unwrap();

    let _data = kiln.accounts().get_by_uuid(uuid);

    Ok(())
}
```
Find complete examples in the `examples` directory.

## License

This package is source available software licensed under the [BUSL-1.1 license](https://github.com/Neal-C/kilnfi-sdk-rs/blob/main/LICENSE).

## Sponsoring

Send some love ❤️ : 0xc58f55f299dF3A26992a2a0823e5A8f073E3C812