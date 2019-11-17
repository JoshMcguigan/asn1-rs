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

        // The first field name is +2 tokens from the SEQUENCE keyword.
        let mut next_field_name = index + 2;
        loop {
            let field_name = tokens[next_field_name];
            let field_type_start_index = next_field_name + 1;
            let field_type_end_index = &tokens[field_type_start_index..]
                .iter()
                .position(|s| s == &"," || s == &"}")
                .unwrap()
                + field_type_start_index;
            let field_type = &tokens[field_type_start_index..field_type_end_index];

            fields.push(AsnField {
                name: field_name,
                field_type: field_type.into(),
            });

            if tokens[field_type_end_index] == "}" {
                // end of fields
                break;
            }

            next_field_name = field_type_end_index + 1;
        }

        let sequence = AsnSequence { fields };

        (sequence_name, sequence)
    }
}

#[derive(Debug, PartialEq)]
pub enum AsnType<'a> {
    /// ASN1 default integer type with no bounds specified
    Integer,
    /// ASN1 default integer type with user specified bounds
    BoundedInteger { min: i128, max: i128 },
    /// Custom type defined by the user
    Custom(&'a str),
}

/// Expected input: "(0..255)"
/// Output: (min, max)
fn parse_bounds(s: &str) -> (i128, i128) {
    let mut vals = s.split("..");
    let min_as_string = vals.next().unwrap().trim_start_matches('(');
    let max_as_string = vals.next().unwrap().trim_end_matches(')');
    assert_eq!(None, vals.next());

    (
        min_as_string.parse().unwrap(),
        max_as_string.parse().unwrap(),
    )
}

impl<'a, 'b> From<&'a [&'b str]> for AsnType<'b> {
    fn from(s: &'a [&'b str]) -> Self {
        match s {
            ["INTEGER"] => Self::Integer,
            [other] => Self::Custom(other),
            ["INTEGER", bounds] => {
                let (min, max) = parse_bounds(bounds);
                Self::BoundedInteger { min, max }
            }
            _ => unimplemented!(),
        }
    }
}

fn tokenizer(s: &str) -> Vec<&str> {
    let tokens: Vec<&str> = s
        .split_whitespace()
        .flat_map(|s| split_keep_separator(s))
        .collect();

    tokens
}

/// Like the std lib split function, but allows us to keep the
/// separator (a comma in our case).
fn split_keep_separator(s: &str) -> Vec<&str> {
    let mut out = vec![];

    let mut slice_start = 0;
    let separator = ',';
    let separator_size = separator.len_utf8();

    for (index, character) in s.char_indices() {
        if character == separator {
            if index > slice_start {
                // push characters before separator, if there are any
                out.push(&s[slice_start..index]);
            }
            // push separator
            let index_behind_separator = index + separator_size;
            out.push(&s[index..index_behind_separator]);
            slice_start = index_behind_separator;
        }
    }

    // push any characters after the last separator
    if slice_start < s.len() {
        out.push(&s[slice_start..s.len()]);
    }

    out
}

impl<'a> From<&'a str> for AsnModule<'a> {
    fn from(s: &'a str) -> Self {
        let tokens: Vec<&str> = tokenizer(s);
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
    fn split_keep_separator() {
        let input = ",";
        assert_eq!(vec![","], super::split_keep_separator(input));

        let input = "test,";
        assert_eq!(vec!["test", ","], super::split_keep_separator(input));

        let input = "test,test2";
        assert_eq!(
            vec!["test", ",", "test2"],
            super::split_keep_separator(input)
        );
    }

    #[test]
    fn tokenizer() {
        let input = "my fake, input";
        assert_eq!(vec!["my", "fake", ",", "input"], super::tokenizer(input));
    }

    #[test]
    fn asn_parse() {
        let asn1_string = include_str!("../../test-asn/geo.asn");
        let asn_module = AsnModule::from(&*asn1_string);

        assert_eq!("Geometry", asn_module.name);
        assert_eq!(4, asn_module.sequences.len());

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

        let rectangle = asn_module.sequences.get("Rectangle").unwrap();
        assert_eq!(2, rectangle.fields.len());
        assert_eq!("width", rectangle.fields[0].name);
        assert_eq!(
            AsnType::BoundedInteger {
                min: 0,
                max: 18_446_744_073_709_551_615
            },
            rectangle.fields[0].field_type
        );
        assert_eq!("height", rectangle.fields[1].name);
        assert_eq!(
            AsnType::BoundedInteger {
                min: 0,
                max: 18_446_744_073_709_551_615
            },
            rectangle.fields[1].field_type
        );

        let tiny_rectangle = asn_module.sequences.get("TinyRectangle").unwrap();
        assert_eq!(2, tiny_rectangle.fields.len());
        assert_eq!("width", tiny_rectangle.fields[0].name);
        assert_eq!(
            AsnType::BoundedInteger { min: 0, max: 255 },
            tiny_rectangle.fields[0].field_type
        );
        assert_eq!("height", tiny_rectangle.fields[1].name);
        assert_eq!(
            AsnType::BoundedInteger { min: 0, max: 255 },
            tiny_rectangle.fields[1].field_type
        );
    }
}
