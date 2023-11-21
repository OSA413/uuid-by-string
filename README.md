uuid-by-string
[![NPM](https://img.shields.io/crates/v/uuid-by-string)](https://crates.io/crates/uuid-by-string)
[![Downloads](https://img.shields.io/crates/dr/uuid-by-string)](https://crates.io/crates/uuid-by-string)
=======================
Generates the [RFC-4122](https://tools.ietf.org/html/rfc4122#section-4.3) Name-Based UUID. Supports 3 and 5 versions of UUID.

This library is rewritten from [Danakt Saushkin's JavaScript library of the same name](https://github.com/danakt/uuid-by-string). All features and tests are in place.

## Installation

```bash
cargo add uuid-by-string
```

## Usage

```rust
use uuid_by_string::generate_uuid::{generate_uuid};
generate_uuid("hello world")
//"2aae6c35-c94f-5fb4-95db-e95f408b9ce9";

// For namespace generation enable feature "namespaces"
use uuid_by_string::generate_uuid_with_namespace::{generate_uuid_with_namespace};
generate_uuid_with_namespace("hello world", "d3486ae9-136e-5856-bc42-212385ea7970").unwrap()
//"1825ed38-348f-5b46-99de-fd84b83aba5e"
```

The string `hello world` will always return`5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3`.

You can specify the UUID version. Available versions is 3 and 5 according to [RFC-4122](https://tools.ietf.org/html/rfc4122#section-4.3). The version is responsible for the hashing algorithm: version 3 uses MD5, and version 5 uses SHA-1. UUIDv5 is used by default if version is not specified.


```rust
use uuid_by_string::generate_uuid::{generate_uuid_v3, generate_uuid_v5};

// For namespace generation enable feature "namespaces"
use uuid_by_string::generate_uuid_with_namespace::{generate_uuid_with_namespace_v3, generate_uuid_with_namespace_v5};

fn main() {
    assert_eq!(generate_uuid_v3("hello world"), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
    assert_eq!(generate_uuid_v5("hello world"), "2aae6c35-c94f-5fb4-95db-e95f408b9ce9");

    // For namespace generation enable feature "namespaces"
    assert_eq!(generate_uuid_with_namespace("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
}
```
