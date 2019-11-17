use std::collections::HashMap;

pub struct AsnModule<'a> {
    name: &'a str,
    pub sequences: HashMap<&'a str, AsnSequence<'a>>,
}

pub struct AsnSequence<'a> {
    // Needs to be a vec to maintain field order
    pub fields: Vec<AsnField<'a>>,
}

pub struct AsnField<'a> {
    pub name: &'a str,
    pub field_type: AsnType<'a>,
}

impl<'a> AsnSequence<'a> {
    /// Parses an ASN SEQUENCE given the tokens and the index
    /// into the tokens slice where the SEQUENCE keyword occurs.
    /// Returns a tuple of the sequence name and the sequence.
    fn from_tokens(tokens: &[&'a str], index: usize) -> (&'a str, Self) {
        let sequence_name_index = index - 2;
        let sequence_name = tokens[sequence_name_index];
        let mut fields = vec![];

        // Offset is the change in token index needed to get from the SEQUENCE
        // keyword to the next field name to process. It is initialized to 2
        // because the first field name is +2 from the SEQUENCE keyword.
        let mut offset = 2;
        loop {
            let field_name = tokens[index + offset];
            // trim the comma between lines
            let field_type = tokens[index + offset + 1].trim_end_matches(',');

            if field_name == "}" {
                // end of fields
                break;
            }

            fields.push(AsnField {
                name: field_name,
                field_type: field_type.into(),
            });

            offset += 2;
        }

        let sequence = AsnSequence { fields };

        (sequence_name, sequence)
    }
}

#[derive(Debug, PartialEq)]
pub enum AsnType<'a> {
    /// ASN1 default integer type with no bounds specified
    Integer,
    /// Custom type defined by the user
    Custom(&'a str),
}

impl<'a> From<&'a str> for AsnType<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "INTEGER" => Self::Integer,
            other => Self::Custom(other),
        }
    }
}

impl<'a> From<&'a str> for AsnModule<'a> {
    fn from(s: &'a str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let name = tokens[0];

        let sequence_indexes: Vec<usize> = tokens
            .iter()
            .enumerate()
            .filter_map(|(i, elem)| if elem == &"SEQUENCE" { Some(i) } else { None })
            .collect();
        let mut sequences = HashMap::new();
        for sequence_index in sequence_indexes {
            let (sequence_name, sequence) = AsnSequence::from_tokens(&tokens, sequence_index);
            sequences.insert(sequence_name, sequence);
        }
        Self { name, sequences }
    }
}

#[cfg(test)]
mod tests {
    use super::{AsnModule, AsnType};

    #[test]
    fn it_works() {
        let asn1_string = include_str!("../../test-asn/geo.asn");
        let asn_module = AsnModule::from(&*asn1_string);

        assert_eq!("Geometry", asn_module.name);
        assert_eq!(2, asn_module.sequences.len());

        let point = asn_module.sequences.get("Point").unwrap();
        assert_eq!(2, point.fields.len());
        assert_eq!("x", point.fields[0].name);
        assert_eq!(AsnType::Integer, point.fields[0].field_type);
        assert_eq!("y", point.fields[1].name);
        assert_eq!(AsnType::Integer, point.fields[1].field_type);

        let line = asn_module.sequences.get("Line").unwrap();
        assert_eq!(2, line.fields.len());
        assert_eq!("p1", line.fields[0].name);
        assert_eq!(AsnType::Custom("Point"), line.fields[0].field_type);
        assert_eq!("p2", line.fields[1].name);
        assert_eq!(AsnType::Custom("Point"), line.fields[1].field_type);
    }
}
