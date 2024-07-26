uuid-by-string
[![Crates.io](https://img.shields.io/crates/v/uuid-by-string)](https://crates.io/crates/uuid-by-string)
[![Downloads](https://img.shields.io/crates/dr/uuid-by-string)](https://crates.io/crates/uuid-by-string)
=======================
Generates the [RFC-4122](https://tools.ietf.org/html/rfc4122#section-4.3) Name-Based UUID. Supports 3 and 5 versions of UUID with and without (non-standard, see below) a namespace.

## 🚨 Warning: Don't use unless explicitly necessary (see [replacement](#replacement)) 🚨

**Note**: generating UUID v3 and v5 without a namespace is **non-standard** (the RFC-4122 covers only the case when you concatenate the namespace with the name, so if you want a reproducable result in other progrmming langiages you need to generate UUID with some namespace, e.g. [nil](https://en.wikipedia.org/wiki/Universally_unique_identifier#Special_UUIDs))

According to the implementation differences, it's impossible to replicate results of the no-namespace UUID generation with a standard generation, so keep that in mind.

This library is rewritten from [Danakt Saushkin's JavaScript library of the same name](https://github.com/danakt/uuid-by-string). All features and tests are in place.

## Installation

```bash
cargo add uuid-by-string
```

## Usage

```rust
use uuid_by_string::generate_uuid::{generate_uuid};
// Note: generating UUID v3 and v5 without namespace is non-standard
generate_uuid("hello world")
//"2aae6c35-c94f-5fb4-95db-e95f408b9ce9";

// For namespace generation enable feature "namespaces"
use uuid_by_string::generate_uuid_with_namespace::{generate_uuid_with_namespace};
generate_uuid_with_namespace("hello world", "d3486ae9-136e-5856-bc42-212385ea7970").unwrap()
//"1825ed38-348f-5b46-99de-fd84b83aba5e"
```

The string `hello world` will always return `2aae6c35-c94f-5fb4-95db-e95f408b9ce9`.

You can specify the UUID version. Available versions is 3 and 5 according to [RFC-4122](https://tools.ietf.org/html/rfc4122#section-4.3). The version is responsible for the hashing algorithm: version 3 uses MD5, and version 5 uses SHA-1. UUIDv5 is used by default if version is not specified.

```rust
use uuid_by_string::generate_uuid::{generate_uuid_v3, generate_uuid_v5};

// For namespace generation enable feature "namespaces"
use uuid_by_string::generate_uuid_with_namespace::{generate_uuid_with_namespace_v3, generate_uuid_with_namespace_v5};

fn main() {
    // Note: generating UUID v3 and v5 without namespace is non-standard
    assert_eq!(generate_uuid_v3("hello world"), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
    assert_eq!(generate_uuid_v5("hello world"), "2aae6c35-c94f-5fb4-95db-e95f408b9ce9");

    // For namespace generation enable feature "namespaces"
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
}
```

## Replacement

You can (and should) replace this library with https://docs.rs/uuid/. The code whould look like this:

```rust
let uuid = Uuid::new_v3(&Uuid::nil(), b"Hello world!");
let uuid = Uuid::new_v5(&Uuid::nil(), b"Hello world!");
```

More info about replacement:
* https://docs.rs/uuid/
* https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v3
* https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v5