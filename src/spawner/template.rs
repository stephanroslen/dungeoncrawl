use crate::prelude::*;
use serde::Deserialize;

use legion::systems::CommandBuffer;
use ron::de::from_reader;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: Frequency,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum Frequency {
    Repeated(usize),
    Once,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        resources: &mut Resources,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|entity| entity.levels.contains(&level))
            .for_each(|template| match template.frequency {
                Frequency::Repeated(n) => {
                    for _ in 0..n {
                        available_entities.push(template);
                    }
                }
                Frequency::Once => {
                    available_entities.push(template);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        let mut spawn_once_already_spawned = HashSet::new();
        spawn_points.iter().for_each(|point| {
            let entity = loop {
                let entity = *rng.random_slice_entry(&available_entities).unwrap();
                match entity.frequency {
                    Frequency::Repeated(_) => {
                        break entity;
                    }
                    Frequency::Once => {
                        if spawn_once_already_spawned.contains(&entity.name) {
                            continue;
                        }
                        spawn_once_already_spawned.insert(entity.name.clone());
                        break entity;
                    }
                }
            };
            self.spawn_entity(*point, entity, &mut commands);
        });
        commands.flush(ecs, resources);
    }

    pub fn spawn_entity(&self, point: Point, template: &Template, commands: &mut CommandBuffer) {
        let entity = commands.push((
            point,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name {
                name: template.name.clone(),
            },
        ));

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage { damage: *damage });
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }

        match template.entity_type {
            EntityType::Item => {
                commands.add_component(entity, Item {});
                if let Some(effects) = &template.provides {
                    effects
                        .iter()
                        .for_each(|(provides, n)| match provides.as_str() {
                            "Healing" => {
                                commands.add_component(entity, ProvidesHealing { amount: *n })
                            }
                            "Depletion" => commands.add_component(entity, ProvidesDepletion {}),
                            "DestructionOnLevelProgress" => commands
                                .add_component(entity, ProvidesDestructionOnLevelProgress {}),
                            "Equipment" => commands.add_component(entity, ProvidesEquipment {}),
                            "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                            _ => {
                                println!("Warning: we don't knpow how to provide {}", provides)
                            }
                        });
                }
            }
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(
                    entity,
                    RoamingAndChasingPlayer {
                        home_location: point,
                        going_to: None,
                    },
                );
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }
    }
}
