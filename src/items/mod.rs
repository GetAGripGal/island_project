use self::inventory::Inventory;
use crate::prelude::{Collider, CollidingEntities, PhysicsBodyBundle};
use bevy::prelude::*;
use hashbrown::HashMap;
use std::ops::{Deref, DerefMut};
pub mod inventory;

/// The registry of items
#[derive(Debug, Default, Clone)]
pub struct ItemRegistry(pub HashMap<String, ItemDescriptor>);

impl Deref for ItemRegistry {
    type Target = HashMap<String, ItemDescriptor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The category of an item
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    Misc,
    Food,
    PowerUp,
    Weapon,
    Key,
}

impl Default for ItemCategory {
    fn default() -> Self {
        Self::Misc
    }
}

/// Describes an item in the inventory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDescriptor {
    pub name: String,
    pub description: String,
    pub icon: Handle<Image>,
    pub category: ItemCategory,
}

/// The item that the entity represents
#[derive(Debug, Clone, Component)]
pub struct RepresentingItem(pub ItemDescriptor);

/// An event that spawns an item entity
#[derive(Debug, Clone)]
pub struct SpawnItemEvent {
    pub item: String,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Deref for RepresentingItem {
    type Target = ItemDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The plugin for managing items
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemRegistry>();
        app.add_event::<SpawnItemEvent>();
        app.add_startup_system(register_items);
        app.add_system(handle_item_collisions);
        app.add_system(handle_item_spawning);
    }
}

/// Register the items in the inventory
fn register_items(asset_server: Res<AssetServer>, mut item_registry: ResMut<ItemRegistry>) {
    // Register the mirror item
    item_registry.insert(
        "mirror".into(),
        ItemDescriptor {
            name: "Mirror".to_string(),
            description: "A cute little mirror, in the impractical shape of a circle".into(),
            icon: asset_server.load("sprites/items/misc/mirror.png"),
            category: ItemCategory::Misc,
        },
    );
    item_registry.insert(
        "monocle".into(),
        ItemDescriptor {
            name: "Monocle".to_string(),
            description: "For people who are only half sightless.".into(),
            icon: asset_server.load("sprites/items/misc/monocle.png"),
            category: ItemCategory::Misc,
        },
    );
    // Register the bean
    item_registry.insert(
        "bean".into(),
        ItemDescriptor {
            name: "Bean".to_string(),
            description: "A bean, for those who like beans.".into(),
            icon: asset_server.load("sprites/items/food/bean.png"),
            category: ItemCategory::Food,
        },
    );
}

/// Handle the collisions between the item entities and an inventory holder
fn handle_item_collisions(
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>,
    item_entities: Query<(Entity, &RepresentingItem, &CollidingEntities)>,
) {
    item_entities.for_each(|(entity, item, colliding_entities)| {
        // If the item is colliding with an inventory holder
        if !colliding_entities.0.is_empty() {
            // Loop over the colliding entities
            colliding_entities.0.iter().for_each(|colliding_entity| {
                // Add the item to the inventory
                if let Ok(mut inventory) = inventories.get_mut(*colliding_entity) {
                    inventory.add_item(item.0.clone());
                    // Destroy the item
                    commands.entity(entity).despawn();
                }
            });
        }
    });
}

/// Handle the item spawning event
fn handle_item_spawning(
    item_registry: ResMut<ItemRegistry>,
    mut commands: Commands,
    mut item_spawning: EventReader<SpawnItemEvent>,
) {
    for event in item_spawning.iter() {
        // Get the item
        let item = item_registry
            .get(&event.item)
            .expect(&format!("Item not found: {}", event.item));
        // Spawn the entity
        commands
            .spawn_bundle(SpriteBundle {
                texture: item.icon.clone(),
                ..Default::default()
            })
            .insert_bundle(PhysicsBodyBundle {
                velocity: event.velocity.into(),
                friction: Vec2::new(3.0, 0.0).into(),
                ..Default::default()
            })
            .insert(Collider {
                tags: vec!["item".into()],
                half_extents: Vec2::new(6.0, 6.0),
                ..Default::default()
            })
            .insert(Transform::from_xyz(event.position.x, event.position.y, 0.0))
            .insert(RepresentingItem(item.clone()))
            .insert(CollidingEntities::default());
    }
}
