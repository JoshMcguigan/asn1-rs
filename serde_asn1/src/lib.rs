mod error;
use error::{Error, Result};

use serde::{de::Visitor, Deserialize};

struct OerDeserializer<'de> {
    input: &'de [u8],
}

impl<'de> OerDeserializer<'de> {
    fn from_oer_bytes(input: &'de [u8]) -> Self {
        Self { input }
    }
}

pub fn from_oer_bytes<'a, T>(input: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = OerDeserializer::from_oer_bytes(input);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut OerDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Message(String::from("no any support")))
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    /// Rec.ITU-T X.696 
    /// TODO convert this to i128 behavior
    /// because values which actually fit into i64 are serialized into fixed size
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_field_size, rest) = self.input.split_first().ok_or(Error::Eof)?;
        self.input = rest;
        // ignore field_size for now
        // but later decide which deserialize methods to use based on length
        let (value, rest) = self.input.split_first().ok_or(Error::Eof)?;
        self.input = rest;
        visitor.visit_i64(i64::from(*value as i8))
    }

    /// Rec.ITU-T X.696 10.3 a
    /// For lower bound greater than or equal to zero.
    /// If the upper bound is less than or equal to 2^8 - 1, then
    /// every value of the integer type shall be encoded as a
    /// fixed-size unsigned number in a one-octet word.
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (value, rest) = self.input.split_first().ok_or(Error::Eof)?;
        self.input = rest;
        visitor.visit_u8(*value)
    }

    /// Rec.ITU-T X.696 10.3 b
    /// For lower bound greater than or equal to zero.
    /// If the upper bound is less than or equal to 2^16 - 1, then
    /// every value of the integer type shall be encoded as a
    /// fixed-size unsigned number in a one-octet word.
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (value_bytes, rest) = self.input.split_at(2);
        self.input = rest;

        // interpret value_bytes as big endian
        let val1 = value_bytes[0];
        let val2 = value_bytes[1];
        let value = u16::from(val1) << 8 | u16::from(val2);
       
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        struct Access<'a, 'de> {
            deserializer: &'a mut OerDeserializer<'de>,
            len: usize,
        }

        impl<'a, 'de> serde::de::SeqAccess<'de> for Access<'a, 'de> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::from_oer_bytes;

    asn1_codegen::from!("../test-asn/geo.asn");

    /// Subprocess call to asn1tools to serialize the given struct using
    /// OER encoding.
    fn serialize_with_asn1tools<'a, T>(
        file_path: &str,
        struct_name: &str,
        struct_under_test: &'a T,
    ) -> Vec<u8>
    where
        T: serde::Serialize,
    {
        let std_out_bytes = std::process::Command::new("asn1tools")
            .args(&[
                "convert",
                file_path,
                struct_name,
                "-o",
                "oer", // output OER format
                "-i",
                "jer", // input JSON format
                // asn1tools expects the input in hex
                &hex::encode(serde_json::to_string(struct_under_test).unwrap().as_bytes()),
            ])
            .current_dir(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .output()
            .unwrap()
            .stdout;
        let std_out_hex_string = String::from_utf8_lossy(&std_out_bytes);
        let std_out_hex_string_without_newline = std_out_hex_string.trim_end();

        hex::decode(std_out_hex_string_without_newline).unwrap()
    }

    #[test]
    fn point() {
        // explicitly specify the types of these fields to verify the code generation
        let point = Point {
            x: -2_i64,
            y: 2_i64,
        };

        let oer_bytes = serialize_with_asn1tools("../test-asn/geo.asn", "Point", &point);

        // This assertion is checking the output of asn1tools, as well as our processing
        // and interpretation of the returned bytes. It is left here mostly because it is
        // helpful to see the input bytes to the deserialization code under test.
        assert_eq!(oer_bytes, [1, 254, 1, 2]);

        assert_eq!(from_oer_bytes::<Point>(&oer_bytes).unwrap(), point);
    }

    #[test]
    fn line() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: 15, y: 25 };
        let line = Line { p1, p2 };

        let oer_bytes = serialize_with_asn1tools("../test-asn/geo.asn", "Line", &line);

        // Sanity check the asn1tools output
        assert_eq!(oer_bytes, [1, 5, 1, 10, 1, 15, 1, 25]);

        assert_eq!(from_oer_bytes::<Line>(&oer_bytes).unwrap(), line);
    }

    #[test]
    fn tiny_rectangle() {
        // explicitly specify the types of these fields to verify the code generation
        let tiny_rectangle = TinyRectangle {
            width: 10_u8,
            height: 5_u8,
        };

        let oer_bytes =
            serialize_with_asn1tools("../test-asn/geo.asn", "TinyRectangle", &tiny_rectangle);

        // Sanity check the asn1tools output
        assert_eq!(oer_bytes, [10, 5]);

        assert_eq!(
            from_oer_bytes::<TinyRectangle>(&oer_bytes).unwrap(),
            tiny_rectangle
        );
    }

    #[test]
    fn small_rectangle() {
        // explicitly specify the types of these fields to verify the code generation
        let small_rectangle = SmallRectangle {
            width: 11_u16,
            height: 6_u16,
        };

        let oer_bytes =
            serialize_with_asn1tools("../test-asn/geo.asn", "SmallRectangle", &small_rectangle);

        // Sanity check the asn1tools output
        assert_eq!(oer_bytes, [0, 11, 0, 6]);

        assert_eq!(
            from_oer_bytes::<SmallRectangle>(&oer_bytes).unwrap(),
            small_rectangle
        );
    }
}
