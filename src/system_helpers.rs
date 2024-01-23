use crate::components::*;

pub fn spawn_item(item_id: &str, item_x: i32, item_y: i32, queue: &mut Vec<ItemToSpawn>) {
    queue.push(ItemToSpawn {
        item_id: item_id.to_string(),
        spawn_pos: Position {
            x: item_x,
            y: item_y,
        },
    })
}

pub fn attach_sprite(
    commands: &mut Commands,
    entity: Entity,
    article: &String,
    spriteinfores: &HashMap<String, SpriteInfo>,
    tileindexres: &HashMap<i32, TileIndex>,
    tilesetsres: &HashMap<String, Handle<TextureAtlas>>,
    z_iterator: &f32,
) -> Entity {
    let sprite_info = spriteinfores.get(article);

    let sprite_data = match sprite_info {
        None => {
            panic!("NO SPRITE INFO");
        }

        Some(x) => x,
    };

    let tile_index = tileindexres.get(&sprite_data.fg);

    let tile_data = match tile_index {
        None => {
            panic!("NO TILE INFO");
        }

        Some(x) => x,
    };

    let tset = tilesetsres.get(&tile_data.tileset_name);

    match tset {
        Some(tsett) => {
            // Value is present

            println!("beepbooooop");

            let skin = commands
                .spawn(ClothingSpriteBundle {
                    spritesheetbundle: SpriteSheetBundle {
                        texture_atlas: tsett.clone(),
                        sprite: TextureAtlasSprite::new(tile_data.tileset_index as usize),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, *z_iterator)), //clothing layers as Z
                        ..default()
                    },
                    prevglobtrans: PreviousGlobalTransform {
                        ..Default::default()
                    },
                })
                .id();

            commands.entity(entity).add_child(skin);
        }
        None => {
            // No value
            println!("No value");
        }
    }

    return entity;
}
