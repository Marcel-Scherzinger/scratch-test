use std::borrow::Cow;

use crate::interpret_json::FormatError;

#[allow(unused)]
pub trait GetOpcodeUnit {
    type Opcode;

    fn get_opcode(&self) -> Self::Opcode;
}

pub(crate) trait FromJsonBlock {
    fn from_json_block(
        opcode: &str,
        inputs: &serde_json::Map<String, serde_json::Value>,
        fields: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<Option<Self>, super::ParseKindError>
    where
        Self: Sized;
}

pub(crate) trait ValueAttrJsonElemtype {
    const ELEMTYPE: &'static str;
}

pub(crate) trait ValueAttributeFromJson: ValueAttrJsonElemtype {
    fn value_from_json_outer(
        source_name: &'static str,
        source_object: &serde_json::Map<String, serde_json::Value>,
        key: Cow<'static, str>,
    ) -> Result<Self, crate::blocks::BlockAttrError>
    where
        Self: Sized,
    {
        if let Some(entry) = &source_object.get(key.as_ref()) {
            match Self::value_from_json(entry) {
                Ok(o) => Ok(o),
                Err(error) => Err(crate::blocks::BlockAttrError::Invalid {
                    treated_as: <Self as ValueAttrJsonElemtype>::ELEMTYPE,
                    attr_name: key,
                    source: source_name,
                    error,
                }),
            }
        } else {
            Err(crate::blocks::BlockAttrError::Missing {
                treated_as: <Self as ValueAttrJsonElemtype>::ELEMTYPE,
                attr_name: key,
                source: source_name,
            })
        }
    }
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized;
}

impl<T: ValueAttributeFromJson> ValueAttributeFromJson for Option<T>
where
    Option<T>: ValueAttrJsonElemtype,
{
    fn value_from_json_outer(
        source_name: &'static str,
        source_object: &serde_json::Map<String, serde_json::Value>,
        key: Cow<'static, str>,
    ) -> Result<Self, crate::blocks::BlockAttrError>
    where
        Self: Sized,
    {
        if let Some(entry) = &source_object.get(key.as_ref()) {
            match T::value_from_json(entry) {
                Ok(o) => Ok(Some(o)),
                Err(crate::interpret_json::FormatError::OpcodeNull) => Ok(None),
                Err(error) => Err(crate::blocks::BlockAttrError::Invalid {
                    treated_as: <Self as ValueAttrJsonElemtype>::ELEMTYPE,
                    attr_name: key,
                    source: source_name,
                    error,
                }),
            }
        } else {
            Ok(None)
        }
    }

    fn value_from_json(_value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        // this function should never be invoked
        Err(FormatError::OpcodeNull)
    }
}
