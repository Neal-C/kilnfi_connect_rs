> [!NOTE]
> follows v1.6 API version

## Description

The Rust Client for the Kiln connect API (v.1.6)

Just HTTPS requests and static typing for an ergonomic developer experience.

Expect breaking changes (and renamings) while below v1.0.0.

Focus is on ease of use and developer experience. Point innacuries in issues or Direct Messages.

Developed on a strict no-unwrap policy.

## Documentation

https://connect-rs-docs.vercel.app/docs/connect_rs/

### Kiln documentation
Kiln documentation : https://docs.api.kiln.fi/docs/quickstart
Kiln API reference documentation : https://docs.api.kiln.fi/reference/getaccounts

## Requirements

- a kiln api token. Please contact support@kiln.fi to get one.

## Installation

```shell
cargo add kilnfi_connect_rs
```

you may need to use the uuid crate

```shell
cargo add uuid --features v4,serde
```
you may need to use the chrono crate

```shell
cargo add chrono --features serde
```


## Example
```rs
use kilnfi_connect_rs::Kiln;
use uuid::Uuid;
// required to create a uuid from a &'str
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kiln_api_token: String = std::env::var("KILN_API_TOKEN")
        .expect("KILN_API_TOKEN is not set in the environment. It is required.");

    let kiln: Kiln = Kiln::builder()
        .api_token(kiln_api_token)
        // no trailing slash, or else it will break
        .base_url("https://api.kiln.fi/v1")
        .seal()
        .build()?;

    let account_id = Uuid::from_str("9e1a6784-9657-4757-b463-8454a34c92b4").unwrap();

    let _data = kiln.accounts().get_by_uuid(account_id).unwrap();

    Ok(())
}
```

Find complete examples in the `examples` directory and the documentation website https://connect-rs-docs.vercel.app/docs/connect_rs/accounts.

## Contributing

I am open to random pull requests that do at least 1 of the following :
- cross items off the roadmap
- fix typos
- fix naming
- add tooling
- add tests
- add/improve documentation
- improve CI/CD

## License

This package is source available software licensed under the [BUSL-1.1 license](https://github.com/Neal-C/kilnfi_connect_rs/blob/main/LICENSE).

## Sponsoring

Send some love ❤️ : 
- ETH : 0xc58f55f299dF3A26992a2a0823e5A8f073E3C812
- SOL : 7H8Q27WfdovL9C1NehfqeGf2KFtaNUXXxTWrRgaaRDh9