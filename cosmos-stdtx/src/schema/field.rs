//! Fields in a type definition

use super::ValueType;
use crate::{msg::Tag, type_name::TypeName};
use serde::{de, Deserialize};
use std::collections::BTreeSet as Set;

/// Fields in an Amino-serialized `sdk.Msg`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Field {
    /// Name of this field
    name: TypeName,

    /// Amino type to serialize this field as
    #[serde(rename = "type")]
    value_type: ValueType,

    /// Field number to use as the key in an Amino message.
    ///
    /// These are all ensured to be `Some` in the `deserialize_vec` method below.
    tag: Option<Tag>,
}

impl Field {
    /// Create a new [`Field`] with the given tag and [`ValueType`]
    pub fn new(name: TypeName, value_type: ValueType, tag: Tag) -> Self {
        Self {
            name,
            tag: Some(tag),
            value_type,
        }
    }

    /// Get the [`TypeName`] for this [`Field`]
    pub fn name(&self) -> &TypeName {
        &self.name
    }

    /// Get the [`ValueType`] for this [`Field`]
    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    /// Get the numerical index [`Tag`] for this [`Field`]
    pub fn tag(&self) -> Tag {
        self.tag.unwrap()
    }
}

/// Deserialize `Vec<Field>`, populating their `tag` if unpopulated
pub(super) fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<Field>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut fields: Vec<Field> = Vec::deserialize(deserializer)?;
    populate_tags(&mut fields).map_err(de::Error::custom)?;
    check_for_duplicate_tags(&fields).map_err(de::Error::custom)?;
    Ok(fields)
}

/// Populate the `tag` for [`Field`] values if unset
fn populate_tags(fields: &mut [Field]) -> Result<(), &str> {
    // Tags are 1-indexed
    let mut tag = 1;

    for field in fields {
        match field.tag {
            Some(t) => {
                if t == 0 {
                    // `0` is not allowed as a field tag
                    return Err("invalid field tag: 0");
                }

                // auto index by last specified tag
                tag = t + 1
            }
            None => {
                field.tag = Some(tag);
                tag += 1;
            }
        }
    }

    Ok(())
}

/// Ensure tags are unique across all fields
pub(super) fn check_for_duplicate_tags(fields: &[Field]) -> Result<(), String> {
    let mut tags = Set::new();

    for field in fields {
        let tag = field.tag.expect("field with unpopulated tag!");

        if !tags.insert(tag) {
            return Err(format!("duplicate field tag: {}", tag));
        }
    }

    Ok(())
}
