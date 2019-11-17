# asn1-rs

#### This crate is a work in progress. It is very incomplete and the ASN.1 file parsing in particular is low quality.

This repo houses a collection of Rust libraries for working with [ASN.1](https://en.wikipedia.org/wiki/Abstract_Syntax_Notation_One).

## Code Generation

```asn1
-- geo.asn

Geometry DEFINITIONS ::= BEGIN

Point ::= SEQUENCE {
	x	INTEGER,
	y	INTEGER
}

Line ::= SEQUENCE {
	p1	Point,
	p2	Point
}

END
```

```toml
# Cargo.toml
[dependencies]
asn1_codegen = "*"
serde_derive = "1.0"
```

```rust
asn1_codegen::from!("geo.asn");

// The macro above will generate the following structs

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq)]
struct Point {
	pub x: i64,
	pub y: i64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq)]
struct Line {
	pub p1: Point,
	pub p2: Point,
}
```

## Deserialization

Deserialization of the [Octet Encoding Rules (OER)](https://www.itu.int/rec/T-REC-X.696-201508-I/en) protocol from bytes into Rust structs is available as demonstrated below.


```toml
# Cargo.toml
[dependencies]
asn1_codegen = "*"
serde_asn1 = "*"
serde_derive = "1.0"
```

```rust
use serde_asn1::from_oer_bytes;

asn1_codegen::from!("geo.asn");

let oer_bytes = [1, 254, 1, 2];
let point = from_oer_bytes::<Point>(&oer_bytes).unwrap();

assert_eq!(2, point.y);
```

### Supported ASN.1 Features

- [x] Structures (SEQUENCE)
- [ ] Lists (SEQUENCE OF)
- [ ] Enumerations (ENUMERATED)
- [ ] Imports (IMPORTS x FROM y)
- [ ] Boolean
- [x] Integer (currently only a subset of constraints are supported)
- [ ] Float

Note that the above is not a complete list of all ASN.1 features.

### Supported ASN.1 Encodings

- [ ] Basic Encoding Rules
- [ ] Distinguished Encoding Rules
- [x] Octet Encoding Rules (decoding support only, for the above listed ASN.1 features)

Note that the above is not a complete list of all ASN.1 encodings.

### Testing

All tests can be run with `cargo test`, but you'll first need to install the [asn1tools](https://pypi.org/project/asn1tools/) package.

Demo ASN.1 files used for testing are kept in the `test-asn` directory of this repo. Parsing of ASN.1 format is done in the `asn1_codegen` crate and includes unit tests in that crate. The `serde_asn1` crate includes end to end tests with the following flow:

```
ASN.1 file 
	-> parsing and code generation
		-> instantiate the struct and serialize it out to asn1tools via json to get it in OER format
			-> OER deserialization
				-> compare original struct to deserialized struct
```

This provides test coverage of parsing, code generation, and OER deserialization. 

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
