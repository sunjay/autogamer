use std::hash::Hash;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::convert::TryInto;

use tiled::PropertyValue;
use thiserror::Error;
use specs::{EntityBuilder, Builder};

use crate::{TileId, Currency};

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("expected `{prop}` property to have type `{expected_type}` (tile GID = {id})")]
    TypeError {
        id: TileId,
        prop: &'static str,
        expected_type: &'static str,
    },

    #[error("expected `{prop}` property to have a value greater than or equal to zero (tile GID = {id})")]
    ExpectedUnsigned {
        id: TileId,
        prop: &'static str,
    },
}

pub trait GetProperty {
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

impl GetProperty for HashMap<String, PropertyValue> {
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

impl<'a> GetProperty for JointHashMap<'a, String, PropertyValue> {
    fn get_prop(&self, prop: &'static str) -> Option<&PropertyValue> {
        self.get(prop)
    }
}

type Template<P> = for<'a> fn(EntityBuilder<'a>, TileId, &str, &P) -> Result<EntityBuilder<'a>, TemplateError>;

/// An extension trait for applying component templates to an entity builder
/// based on tile type and properties
pub trait ApplyComponentTemplates: Sized {
    fn apply_templates<P: GetProperty>(
        self,
        id: TileId,
        tile_type: &str,
        props: &P,
    ) -> Result<Self, TemplateError>;
}

impl<'a> ApplyComponentTemplates for EntityBuilder<'a> {
    fn apply_templates<P: GetProperty>(
        self,
        id: TileId,
        tile_type: &str,
        props: &P,
    ) -> Result<Self, TemplateError> {
        // All template functions must be listed here
        let templates: &[Template<P>] = &[
            currency,
            damage,
        ];

        let mut builder = self;
        for template in templates {
            builder = template(builder, id, tile_type, props)?;
        }

        Ok(builder)
    }
}

pub fn currency<'a, P: GetProperty>(
    builder: EntityBuilder<'a>,
    id: TileId,
    _tile_type: &str,
    props: &P,
) -> Result<EntityBuilder<'a>, TemplateError> {
    Ok(match props.get_u32("currency_value", id) {
        Some(value) => {
            let value = value?;
            builder.with(Currency(value))
        },
        None => builder,
    })
}

pub fn damage<'a, P: GetProperty>(
    builder: EntityBuilder<'a>,
    id: TileId,
    tile_type: &str,
    props: &P,
) -> Result<EntityBuilder<'a>, TemplateError> {
    //TODO
    Ok(builder)
}
