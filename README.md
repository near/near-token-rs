<p>
    <a href="https://crates.io/crates/near-token"><img src="https://img.shields.io/crates/d/near-token?style=flat-square&logo=near&label=crates.io" alt="Crates.io (latest)"></a>
    <a href="https://docs.rs/near-token/latest/near_token"><img src="https://img.shields.io/docsrs/near-token?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

# near-token
near-token is crate for work with [tokens](https://docs.near.org/concepts/basics/tokens) in near-protocol.

The crate includes NearToken type and constructors for converting data as NearToken and as u128 type values.

## Examples

### Basic

Add near-token to your dependencies:

```bash
cargo add near-token
```

Here is the basic usage of near-token crate:

```rust
use near_token::NearToken;

fn main() {
    const TEN_NEAR: NearToken = NearToken::from_near(10);

    assert_eq!(TEN_NEAR.to_string(), "10.00 NEAR");
    assert_eq!(TEN_NEAR.as_near(), 10);
    assert_eq!(TEN_NEAR.as_millinear(), 10000);
    assert_eq!(TEN_NEAR.as_yoctonear(), 10000000000000000000000000);

    let input_str = "0.123456 NEAR";
    let input_near: NearToken = input_str.parse().unwrap();
    assert_eq!(
        input_near,
        NearToken::from_yoctonear(123456000000000000000000)
    );

}
```

### serde support

In order to use NearToken in `serde`-serializable structs, enable `serde` feature:

```bash
cargo add near-token --features serde
```

Here is the basic usage of near-token crate with serde:

```rust
// When `serde` feature is enabled, NearToken can be used in serde-serializable structs.
// NearToken will be serialized to a token-precision u128 value encoded as string.
#[derive(serde::Serialize)]
struct TransferDetails {
    amount: NearToken,
}

fn main() {
    const TEN_NEAR: NearToken = NearToken::from_near(10);

    let details = TransferDetails { amount: TEN_NEAR };
    assert_eq!(
        serde_json::to_string(&details).unwrap(),
        r#"{"amount":"10000000000000000000000000"}"#
    );
}
```

### borsh support

In order to use NearToken in `borsh`-serializable structs, enable `borsh` feature:

```bash
cargo add near-token --features borsh
```

Here is the basic usage of near-token crate with borsh:

```rust
use borsh::{to_vec, BorshSerialize};
use near_token::NearToken;

#[derive(BorshSerialize)]
struct TransferDetails {
    amount: NearToken,
}

fn main() {
    const TEN_NEAR: NearToken = NearToken::from_near(10);

    let details = TransferDetails { amount: TEN_NEAR };
    assert_eq!(
        to_vec(&details).unwrap(),
        vec![0, 0, 0, 74, 72, 1, 20, 22, 149, 69, 8, 0, 0, 0, 0, 0]
    );
}
```

## NearToken information

NEAR is used to price computation and storage on the NEAR infrastructure. The network charges transaction fees in NEAR to process changes and transactions.

### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/near/near-token/blob/main/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/near/near-token/blob/main/LICENSE-APACHE

