use specs::{System, SystemData, World, Read, Entities, ReadStorage, WriteStorage, Join, prelude::ResourceId};

use crate::{CollisionsMap, Currency, Wallet};

#[derive(SystemData)]
pub struct Data<'a> {
    pub collisions: Read<'a, CollisionsMap>,
    pub entities: Entities<'a>,
    pub currencies: ReadStorage<'a, Currency>,
    pub wallets: WriteStorage<'a, Wallet>,
}

/// The system of currency in the game
#[derive(Debug, Default)]
pub struct CurrencySystem;

impl<'a> System<'a> for CurrencySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Data {
            collisions,
            entities,
            currencies,
            mut wallets,
        } = data;

        for (entity, wallet) in (&entities, &mut wallets).join() {
            let Wallet(wallet_value) = wallet;

            let intersecting = &collisions.get(entity).intersecting;
            for &intersecting_entity in intersecting {
                if let Some(&Currency(value)) = currencies.get(intersecting_entity) {
                    *wallet_value += value;

                    // Remove the collected currency entity
                    entities.delete(intersecting_entity)
                        .expect("bug: unable to delete currency entity");
                }
            }
        }
    }
}
