mod custom_props;
mod entity_editor;

pub use custom_props::*;
pub use entity_editor::*;

use thiserror::Error;

use crate::{TileId, Currency, PhysicsCollider};

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

    #[error(transparent)]
    SpecsError(#[from] specs::error::Error),
}

type Template<P> = for<'a> fn(&EntityEditor<'a>, TileId, &str, &P) -> Result<(), TemplateError>;

/// An extension trait for applying component templates to an entity builder
/// based on tile type and properties
pub trait ApplyComponentTemplates {
    fn apply_templates<P: CustomProps>(
        &self,
        id: TileId,
        tile_type: &str,
        props: &P,
    ) -> Result<(), TemplateError>;
}

impl<'a> ApplyComponentTemplates for EntityEditor<'a> {
    fn apply_templates<P: CustomProps>(
        &self,
        id: TileId,
        tile_type: &str,
        props: &P,
    ) -> Result<(), TemplateError> {
        // All template functions must be listed here
        let templates: &[Template<P>] = &[
            currency,
            ladder,
            damage,
        ];

        for template in templates {
            template(self, id, tile_type, props)?;
        }

        Ok(())
    }
}

fn currency<'a, P: CustomProps>(
    entity: &EntityEditor<'a>,
    id: TileId,
    _tile_type: &str,
    props: &P,
) -> Result<(), TemplateError> {
    if let Some(value) = props.get_i32("currency_value", id) {
        let value = value?;
        entity.add(Currency(value))?;
        // Allow entities to pass through this entity
        make_sensor(entity);
    }

    Ok(())
}

fn ladder<'a, P: CustomProps>(
    entity: &EntityEditor<'a>,
    _id: TileId,
    tile_type: &str,
    _props: &P,
) -> Result<(), TemplateError> {
    if tile_type == "ladder" {
        make_sensor(entity);
    }

    Ok(())
}

fn damage<'a, P: CustomProps>(
    entity: &EntityEditor<'a>,
    id: TileId,
    tile_type: &str,
    props: &P,
) -> Result<(), TemplateError> {
    //TODO
    Ok(())
}

/// Retrieves the physics collider component of the given entity and makes it
/// into a sensor. A sensor will not generate contact events, but will generate
/// proximity events. That means that you can interact with it, but it won't
/// stop something from passing through it.
fn make_sensor(entity: &EntityEditor) {
    let mut collider = entity.get_mut::<PhysicsCollider>()
        .expect("bug: all tiles should have a physics collider component");
    collider.sensor = true;
}
