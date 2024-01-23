pub use crate::components::*;
pub use crate::init_helpers::*;
pub use crate::system_helpers::*;

pub fn attach_local_player_component(
    mut commands: Commands,
    lp: ResMut<LocalPlayerResource>,
    player_query: Query<(Entity, &Player)>,
) {
    for (e, player) in &player_query {
        if player.0 == lp.id {
            let local_player_component = LocalPlayerComponent { id: lp.id };
            commands.entity(e).insert(local_player_component);
            commands.remove_resource::<LocalPlayerResource>();
            println!("drasssssssssssssw map is is: {}", lp.id);
        }
    }
}

/// Reads player inputs and sends [`MoveCommandEvents`]
pub fn input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>) {
    let mut direction = MoveDirection::ZERO;
    if input.just_pressed(KeyCode::Right) {
        direction.x += 1;
    }
    if input.just_pressed(KeyCode::Left) {
        direction.x -= 1;
    }
    if input.just_pressed(KeyCode::Up) {
        direction.y += 1;
    }
    if input.just_pressed(KeyCode::Down) {
        direction.y -= 1;
    }
    if direction != MoveDirection::ZERO {
        move_events.send(direction);
    }
}

pub fn movement_system(
    mut move_events: EventReader<FromClient<MoveDirection>>,
    mut players: Query<(&Player, &mut Position)>,
) {
    for FromClient { client_id, event } in move_events.read() {
        info!("received event {event:?} from client {client_id}");
        for (player, mut position) in &mut players {
            if *client_id == player.0 {
                position.x += event.x;
                position.y += event.y;
            }
        }
    }
}

pub fn update_transforms(mut ents: Query<(&Position, &mut Transform)>) {
    for (pos, mut trans) in &mut ents {
        let pix_size: i32 = 32;
        let prev_z = trans.translation.z;
        //     let prev_y = trans.translation.y;
        //     let prev_x = trans.translation.x;

        trans.translation = Vec3 {
            x: (pos.x * pix_size) as f32,
            y: (pos.y * pix_size) as f32,
            z: prev_z,
        };
    }
}

pub fn update_sprites(
    mut ents: Query<(
        &mut TextureAtlasSprite,
        &GlobalTransform,
        &mut PreviousGlobalTransform,
    )>,
) {
    for (mut sprite, trans, mut prev_trans) in &mut ents {
        let pix_size: i32 = 32;

        let prev_x = &prev_trans.translation.x;
        let cur_x = &trans.translation().x;

        if cur_x - prev_x > 1.0 {
            sprite.flip_x = false;
        } else if cur_x - prev_x < -1.0 {
            sprite.flip_x = true;
        }

        prev_trans.translation = trans.translation()
    }
}

pub fn update_camera(
    mut cameras: Query<(&Camera2d, &mut Transform), Without<LocalPlayerComponent>>,
    local_player_query: Query<(&Transform), With<LocalPlayerComponent>>,
) {
    for (cam, mut cam_trans) in &mut cameras {
        for (player_trans) in &local_player_query {
            *cam_trans = player_trans.clone();
        }
    }
}

pub fn spawn_items(
    mut commands: Commands,
    mut itemspawnqueue_res: ResMut<ItemSpawnQueue>,
    tilesets_res: Res<Tilesets>,
    spriteinfomap_res: Res<SpriteInfoMap>,
    tileindexmap_res: Res<TileIndexMap>,
    jsondata_res: Res<SmartData>,
) {
    loop {
        match itemspawnqueue_res.item_spawn_queue.pop() {
            Some(x) => {
                let id = x.item_id;
                let pos = x.spawn_pos;
                let primar = commands
                    .spawn((CatItemBundle {
                        item_details: ItemDetails {
                            item_id: id.clone(),
                        },
                        pos: pos,
                        ..Default::default()
                    },))
                    .id();
                let z_iterator = 0.01;

                attach_sprite(
                    &mut commands,
                    primar,
                    &id,
                    &spriteinfomap_res.simap,
                    &tileindexmap_res.timap,
                    &tilesets_res.named_tilesets,
                    &z_iterator,
                );

                println!("{}", primar.index());
            }

            None => break,
        }
    }
}

pub fn spawn_test_map(
    mut commands: Commands,
    tilesets_res: Res<Tilesets>,
    spriteinfomap_res: Res<SpriteInfoMap>,
    tileindexmap_res: Res<TileIndexMap>,
    mut itemspawnqueue_res: ResMut<ItemSpawnQueue>,
) {
    let map_size_x: i32 = 100;
    let map_size_y: i32 = 100;
    let z_level = 0;
    let pix_size: i32 = 32;

    spawn_item(
        "needle_bone",
        5,
        5,
        &mut itemspawnqueue_res.item_spawn_queue,
    );
    spawn_item(
        "needle_bone",
        7,
        7,
        &mut itemspawnqueue_res.item_spawn_queue,
    );
    for x in 0..map_size_x {
        for y in 0..map_size_y {
            let article = "t_concrete";

            let sprite_info = spriteinfomap_res.simap.get(article);

            let sprite_data = match sprite_info {
                None => {
                    panic!("NO SPRITE INFO");
                }

                Some(x) => x,
            };

            let tile_index = tileindexmap_res.timap.get(&sprite_data.fg);

            let tile_data = match tile_index {
                None => {
                    panic!("NO TILE INFO");
                }

                Some(x) => x,
            };

            let tset = tilesets_res.named_tilesets.get(&tile_data.tileset_name);
            match tset {
                Some(tsett) => {
                    commands.spawn(SpriteSheetBundle {
                        texture_atlas: tsett.clone(),
                        sprite: TextureAtlasSprite::new(tile_data.tileset_index as usize),
                        transform: Transform::from_translation(Vec3::new(
                            (x * pix_size) as f32,
                            (y * pix_size) as f32,
                            z_level as f32,
                        )), //clothing layers as Z
                        ..default()
                    });
                }

                None => (),
            }
        }
    }
}

pub fn attach_clothes(
    mut commands: Commands,
    tilesets_res: Res<Tilesets>,
    spriteinfomap_res: Res<SpriteInfoMap>,
    tileindexmap_res: Res<TileIndexMap>,
    query: Query<
        (
            Entity,
            &WornItems,
            &WieldedItems,
            &PhysicalCharacteristics,
            &Position,
        ),
        (Without<Initialized>),
    >,
) {
    for (e, worn, wielded, chars, pos) in &query {
        let mut chars_vec: Vec<String> = vec![];

        if chars.gender == "female".to_string() {
            chars_vec.push("player_female".to_string())
        }
        if chars.gender == "male".to_string() {
            chars_vec.push("player_male".to_string())
        }
        let eye_color = "overlay_mutation_eye_color_var_".to_string() + &chars.eye_color;
        let hair = "overlay_".to_string()
            + &chars.gender
            + "_mutation_hair_"
            + &chars.hair_color
            + "_"
            + &chars.hair_style;
        let skin =
            "overlay_".to_string() + &chars.gender + "_mutation_SKIN_" + &chars.skin.to_uppercase(); //mohawk doesnt have gender

        chars_vec.push(skin);
        chars_vec.push(eye_color);
        chars_vec.push(hair);

        let mut z = worn
            .worn
            .clone()
            .into_iter()
            .map(|x| "overlay_worn_".to_string() + &x)
            .collect();

        let mut zz = wielded
            .wielded
            .clone()
            .into_iter()
            .map(|x| "overlay_wielded_".to_string() + &x)
            .collect();

        chars_vec.append(&mut z);

        chars_vec.append(&mut zz);

        println!("clearing children");

        commands.entity(e).clear_children();
        let rx = (pos.x * 32) as f32;
        let ry = (pos.y * 32) as f32;
        commands
            .entity(e)
            .insert(SpatialBundle::from_transform(Transform::from_translation(
                Vec3::new(rx, ry, 0.1),
            )));
        commands.entity(e).insert(Initialized {});

        let mut z_iterator = 0.0;

        for article in chars_vec {
            z_iterator += 0.0001;

            attach_sprite(
                &mut commands,
                e,
                &article,
                &spriteinfomap_res.simap,
                &tileindexmap_res.timap,
                &tilesets_res.named_tilesets,
                &z_iterator,
            );
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    local_player_query: Query<(&Position), With<LocalPlayerComponent>>,
) {
    egui::SidePanel::right("my_left_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            //  ui.label("Add widgets");

            for (pos) in &local_player_query {
                ui.label(format!("Player Position : ({0},{1})", pos.x, pos.y));
            }
        });
    });
}
