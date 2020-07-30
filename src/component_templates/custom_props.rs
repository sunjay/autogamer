use std::hash::Hash;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::convert::TryInto;

use tiled::PropertyValue;

use crate::TileId;

use super::TemplateError;

/// Provides convenient access to custom property values, including common type
/// conversions
pub trait CustomProps {
    fn get_prop(&self, prop: &'static str) -> Option<&PropertyValue>;

    fn get_i32(&self, prop: &'static str, id: TileId) -> Option<Result<i32, TemplateError>> {
        self.get_prop(prop).map(|value| match value {
            &PropertyValue::IntValue(value) => Ok(value),
            _ => Err(TemplateError::TypeError {id, prop, expected_type: "int"}),
        })
    }

    fn get_u32(&self, prop: &'static str, id: TileId) -> Option<Result<u32, TemplateError>> {
        let value = self.get_i32(prop, id)?;
        Some(value.and_then(|value| value.try_into().map_err(|_| {
            TemplateError::ExpectedUnsigned {id, prop}
        })))
    }
}

impl CustomProps for HashMap<String, PropertyValue> {
    fn get_prop(&self, prop: &'static str) -> Option<&PropertyValue> {
        self.get(prop)
    }
}

pub(crate) struct JointHashMap<'a, K, V> {
    /// The base hashmap, only keys not found in `data` will be looked up here
    pub base: &'a HashMap<K, V>,
    /// The hashmap containing the main data
    pub data: &'a HashMap<K, V>,
}

impl<'a, K, V> JointHashMap<'a, K, V> where
    K: Eq + Hash,
{
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V> where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.data.get(key).or_else(|| self.base.get(key))
    }
}

impl<'a> CustomProps for JointHashMap<'a, String, PropertyValue> {
    fn get_prop(&self, prop: &'static str) -> Option<&PropertyValue> {
        self.get(prop)
    }
}
