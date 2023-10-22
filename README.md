<p>
    <a href="https://crates.io/crates/near-token"><img src="https://img.shields.io/crates/dv/near-token?style=flat-square&logo=near&label=crates.io" alt="Crates.io (latest)"></a>
    <a href="https://docs.rs/near-token/0.0.1/near_token"><img src="https://img.shields.io/docsrs/near-token?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

# near-token
near-token is crate for work with tokens in near-protocol.

The crate includes NearToken type and constructors for converting data as NearToken and as u128 type values.

## near-token examples 
```rust
use near_token::NearToken;

fn main() {
    assert_eq!(NearToken::from_near(10).as_near(), 10);
    assert_eq!(
        NearToken::from_near(1),
        NearToken::from_yoctonear(10u128.pow(24))
    );
    assert_eq!(NearToken::from_yoctonear(10u128.pow(24)).as_near(), 1);
}
```
## NearToken information
NEAR is used to price computation and storage on the NEAR infrastructure. The network charges transaction fees in NEAR to process changes and transactions.
 



### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/Mr0melian/near_gas/blob/master/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/Mr0melian/near_gas/blob/master/LICENSE-APACHE
[For more information]: https://docs.near.org/concepts/basics/transactions/gas
[Gas usege in Near Protocol]: https://nomicon.io/RuntimeSpec/Fees/
