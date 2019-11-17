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
    field_type: AsnType<'a>,
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
                field_type: AsnType { name: field_type },
            });

            offset += 2;
        }

        let sequence = AsnSequence { fields };

        (sequence_name, sequence)
    }
}

// TODO maybe this needs to be an enum of default/custom type
// also need to handle resolving types from other places
pub struct AsnType<'a> {
    name: &'a str,
}

impl<'a> From<&'a str> for AsnModule<'a> {
    fn from(s: &'a str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let name = tokens[0];

        let sequence_index = tokens.iter().position(|elem| elem == &"SEQUENCE").unwrap();
        let mut sequences = HashMap::new();
        let (sequence_name, sequence) = AsnSequence::from_tokens(&tokens, sequence_index);
        sequences.insert(sequence_name, sequence);
        Self { name, sequences }
    }
}

#[cfg(test)]
mod tests {
    use super::AsnModule;

    #[test]
    fn it_works() {
        let asn1_string = include_str!("../../test-asn/point.asn");
        let asn_module = AsnModule::from(&*asn1_string);

        assert_eq!("PointModule", asn_module.name);
        assert_eq!(1, asn_module.sequences.len());
        assert_eq!(2, asn_module.sequences.get("Point").unwrap().fields.len());
        assert_eq!(
            "x",
            asn_module.sequences.get("Point").unwrap().fields[0].name
        );
        assert_eq!(
            "y",
            asn_module.sequences.get("Point").unwrap().fields[1].name
        );
    }
}
