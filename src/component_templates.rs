use std::hash::Hash;
use std::collections::HashMap;
use std::borrow::Borrow;

use tiled::PropertyValue;
use thiserror::Error;
use specs::{EntityBuilder, Builder};

pub trait GetProperty {
    fn get_prop(&self, key: &str) -> Option<&PropertyValue>;
}

impl GetProperty for HashMap<String, PropertyValue> {
    fn get_prop(&self, key: &str) -> Option<&PropertyValue> {
        self.get(key)
    }
}

pub(crate) struct JointHashMap<K, V> {
    /// The base hashmap, only keys not found in `data` will be looked up here
    pub base: HashMap<K, V>,
    /// The hashmap containing the main data
    pub data: HashMap<K, V>,
}

impl<K, V> JointHashMap<K, V> where
    K: Eq + Hash,
{
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V> where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.data.get(key).or_else(|| self.base.get(key))
    }
}

impl GetProperty for JointHashMap<String, PropertyValue> {
    fn get_prop(&self, key: &str) -> Option<&PropertyValue> {
        self.get(key)
    }
}

#[derive(Debug, Error)]
pub enum TemplateError {
}

type Template<P> = for<'a> fn(EntityBuilder<'a>, &str, &P) -> Result<EntityBuilder<'a>, TemplateError>;

/// An extension trait for applying component templates to an entity builder
/// based on tile type and properties
pub trait ApplyComponentTemplates: Sized {
    fn apply_templates<P: GetProperty>(
        self,
        tile_type: &str,
        props: &P,
    ) -> Result<Self, TemplateError>;
}

impl<'a> ApplyComponentTemplates for EntityBuilder<'a> {
    fn apply_templates<P: GetProperty>(
        self,
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
            builder = template(builder, tile_type, props)?;
        }

        Ok(builder)
    }
}

pub fn currency<'a, P: GetProperty>(
    builder: EntityBuilder<'a>,
    tile_type: &str,
    props: &P,
) -> Result<EntityBuilder<'a>, TemplateError> {
    todo!()
}

pub fn damage<'a, P: GetProperty>(
    builder: EntityBuilder<'a>,
    tile_type: &str,
    props: &P,
) -> Result<EntityBuilder<'a>, TemplateError> {
    todo!()
}
