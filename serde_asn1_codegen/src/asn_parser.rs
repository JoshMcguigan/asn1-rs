use std::collections::HashMap;

struct AsnModule<'a> {
    name: &'a str,
    sequences: HashMap<&'a str, AsnSequence<'a>>,
}

struct AsnSequence<'a> {
    fields: HashMap<&'a str, AsnType<'a>>,
}

// TODO maybe this needs to be an enum of default/custom type
// also need to handle resolving types from other places
struct AsnType<'a> {
    name: &'a str,
}

impl<'a> From<&'a str> for AsnModule<'a> {
    fn from(s: &'a str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let name = tokens[0];

        let sequence_index = tokens.iter().position(|elem| elem == &"SEQUENCE").unwrap();
        let sequence_name_index = sequence_index - 2;
        let sequence_name = tokens[sequence_name_index];
        let sequence = AsnSequence { fields: HashMap::new() }; 
        let mut sequences = HashMap::new();
        sequences.insert(sequence_name, sequence);
        Self { name, sequences }
    }
}

#[cfg(test)]
mod tests {
    use super::AsnModule;

    #[test]
    fn it_works() {
        let asn1_string = std::fs::read_to_string("../src/asn/point.asn").unwrap();
        let asn_module = AsnModule::from(&*asn1_string);


        assert_eq!("PointModule", asn_module.name);
        assert_eq!(1, asn_module.sequences.len());
        assert_eq!(2, asn_module.sequences.get("Point").unwrap().fields.len());
    }
}
