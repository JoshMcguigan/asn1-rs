# asn1-rs

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
	pub p1: i64,
	pub p2: i64,
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
