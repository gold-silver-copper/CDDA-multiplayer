use crate::components::*;

pub fn init_json2(mut commands: Commands) {
    let mut item_map: Vec<Value> = Vec::new(); //<"itemtype",<"id",item>>
    let mut serde_data = SERDEdata { data: Vec::new() };
    let mut item_counter = 0;

    //https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    for entry in WalkDir::new("./assets/data/json/")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_str().unwrap();
        let f_exten = match entry.path().extension() {
            Some(exten) => exten.to_str().unwrap(),
            None => "lol",
        };

        if (entry.path().is_file()) && (f_exten == "json") {
            //must be file cause could be directory
            //    println!("json data path: {}", entry.path().display());

            let data = fs::read_to_string(path).expect("Unable to read json data from path");

            let mut json_to_parse: serde_json::Value =
                serde_json::from_str(&data).expect("Unable to parse itemfile from json");

            let mut dataitemarray: &mut Vec<Value> = &mut Vec::new();

            if json_to_parse.is_object() {
                dataitemarray.push(json_to_parse);
            } else if json_to_parse.is_array() {
                dataitemarray = json_to_parse.as_array_mut().unwrap();
            } else {
                panic!("{json_to_parse:#?}");
            }

            let finalarray = dataitemarray;

            for item in finalarray {
                item_counter += 1;

                let ritem = item.take();

                serde_data.data.push(ritem);
            }
        }
    }

    //HERE WE NOW HAVE A VECTOR OF VALUES, EACH VALUE IS A JSON OBJECT

    commands.insert_resource(serde_data);
}

pub fn init_sprite_bundles(mut commands: Commands, tilesetsres: Res<Tilesets>) {
    // parseddatares: Res<ParsedData>
    let path = "./assets/gfx/MshockXotto+/tile_config.json";
    let data = fs::read_to_string(path).expect("Unable to read tile_config.json");
    let tile_config: serde_json::Value =
        serde_json::from_str(&data).expect("Unable to parse tile_config.json");

    let mut my_spritemap: HashMap<String, SpriteInfo> = HashMap::new(); //// sprite_id, ( tile fg index, tile bg index,rotates,multitile)
    let mut tile_index_map: HashMap<i32, TileIndex> = HashMap::new(); //// fg, (tileset name, tile index)

    for tileset in tile_config["tiles-new"].as_array().unwrap() {
        let tiles = tileset["tiles"].as_array().unwrap();
        let file_name = tileset["file"].as_str().unwrap();
        let tileset_name = file_name[0..file_name.len() - 4].to_string();

        let begin_index: i32 = match tileset["//"].as_str() {
            Some(wid) => {
                //println!("wtf lollll: {}",wid.split_whitespace().next().next().unwrap().to_string());
                let mut iter = wid.split_whitespace();
                iter.next();
                iter.next().unwrap().to_string().parse::<i32>().unwrap()
            }
            None => 0,
        };
        let end_index: i32 = match tileset["//"].as_str() {
            Some(wid) => {
                //println!("wtf lollll: {}",wid.split_whitespace().next().next().unwrap().to_string());
                let mut iter = wid.split_whitespace();
                iter.next();
                iter.next();
                iter.next();
                iter.next().unwrap().to_string().parse::<i32>().unwrap()
            }
            None => 0,
        };

        for x in begin_index..=end_index {
            let mut index_counter: i32 = -1;
            if begin_index == 1 {
                index_counter = x
            } else {
                index_counter = x - begin_index;
            }

            tile_index_map.insert(
                x,
                TileIndex {
                    tileset_name: tileset_name.clone(),
                    tileset_index: index_counter.clone(),
                },
            );
        }

        println!("begin index : {} ", begin_index);
        //{ "id": [ "player_female", "npc_female" ], "fg": 5, "rotates": false, "bg": 4816 },
        for tile in tiles {
            let graf_def = tile.as_object().unwrap();

            let tile_id_vec: Vec<String> = match graf_def.get("id") {
                Some(s_id) => {
                    println!("sid{s_id:#?}");

                    let mut return_vec: Vec<String> = Vec::new();

                    if s_id.is_string() {
                        return_vec.push(s_id.as_str().unwrap().to_string());
                    } else if s_id.is_array() {
                        return_vec = s_id
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|x| x.as_str().unwrap().to_string())
                            .collect();
                    } else {
                        panic! {"S ID IS NOT STRING OR ARRAY"};
                    }

                    println!("rvec{return_vec:#?}");

                    return_vec
                }

                None => {
                    panic!("NO ID");
                    let return_vec: Vec<String> = Vec::new();
                    return_vec
                }
            };

            for t_id in tile_id_vec {
                let mut tinfo = SpriteInfo {
                    sprite_id: t_id.clone(),
                    ..default()
                };

                //checking if it is a multitile
                tinfo.fg = match graf_def.get("fg") {
                    None => -1 as i32,
                    Some(x) => {
                        if x.is_i64() {
                            x.as_i64().unwrap() as i32
                        } else {
                            -1 as i32
                        }
                    }
                };

                //checking if it is a multitile
                tinfo.multitile = match graf_def.get("additional_tiles") {
                    None => false,
                    Some(x) => true,
                };

                my_spritemap.insert(t_id.clone(), tinfo);
            }
        }
    }

    // println!("{my_spritemap:#?}");
    commands.insert_resource(TileIndexMap {
        timap: tile_index_map,
    });
    commands.insert_resource(SpriteInfoMap {
        simap: my_spritemap,
    });
}

pub fn init_tilesets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let path = "./assets/gfx/MshockXotto+/tile_config.json";
    let data = fs::read_to_string(path).expect("Unable to read tile_config.json");
    let tile_config: serde_json::Value =
        serde_json::from_str(&data).expect("Unable to parse tile_config.json");
    println!("{}", tile_config["tiles-new"][0]["file"]);

    let tile_info = tile_config["tile_info"][0].as_object().unwrap();

    let default_pixelscale = tile_info["pixelscale"].as_i64().unwrap();
    let default_tile_width = tile_info["width"].as_i64().unwrap() as usize;
    let default_tile_height = tile_info["height"].as_i64().unwrap() as usize;
    let default_x_offset = 0 as f32;
    let default_y_offset = 0 as f32;

    let mut my_map: HashMap<String, Handle<TextureAtlas>> = HashMap::new();

    for tileset in tile_config["tiles-new"].as_array().unwrap() {
        let mut file_name = tileset["file"].as_str().unwrap();
        println!("file_name: {}", file_name);

        let bevy_path = "gfx/MshockXotto+/".to_string() + file_name;
        let rust_path = "./assets/gfx/MshockXotto+/".to_string() + file_name;

        let mut sprite_width = default_tile_width;
        let mut sprite_height = default_tile_height;
        let mut sprite_offset_x = default_x_offset;
        let mut sprite_offset_y = default_y_offset;

        //
        let custom_width = tileset["sprite_width"].as_i64();

        match custom_width {
            Some(wid) => {
                sprite_width = wid as usize;
                println!("custom width: {} ", wid);
            }
            None => println!("no custom width "),
        }

        let custom_height = tileset["sprite_height"].as_i64();

        match custom_height {
            Some(hei) => {
                sprite_height = hei as usize;
                println!("custom hei: {} ", hei);
            }
            None => println!("no custom height "),
        }

        //
        // x offset actually useless rn
        let x_offset = tileset["sprite_offset_x"].as_i64();

        let xo = match x_offset {
            Some(wid) => {
                sprite_offset_x = wid as f32;
                println!("custom offset x: {} ", wid);
            }
            None => println!("no custom width "),
        };

        let y_offset = tileset["sprite_offset_y"].as_i64();

        let yo = match y_offset {
            Some(hei) => {
                sprite_offset_y = hei as f32;
                println!("custom offset y: {} ", hei);
            }
            None => println!("no custom height "),
        };

        let offset = Some(Vec2::new(sprite_offset_x, sprite_offset_y));

        //
        match imagesize::size(rust_path) {
            Ok(size) => {
                let mut tileset_rows = 0;
                let mut tileset_columns = 0;

                let tileset_width = size.width;
                let tileset_height = size.height;
                tileset_rows = tileset_height / sprite_height;
                tileset_columns = tileset_width / sprite_width;
                println!("tileset rows : {} ", tileset_rows);
                println!("tileset cols : {} ", tileset_columns);

                let tiles_handle = asset_server.load(bevy_path);
                let tiles_atlas = TextureAtlas::from_grid(
                    tiles_handle,
                    Vec2::new(sprite_width as f32, sprite_height as f32),
                    tileset_columns,
                    tileset_rows,
                    None,
                    None,
                );
                let tiles_atlas_handle = texture_atlases.add(tiles_atlas);

                // map is <name of file without extension , handle clone>
                my_map.insert(
                    file_name[0..file_name.len() - 4].to_string(),
                    tiles_atlas_handle.clone(),
                );
            }
            Err(why) => panic!("EEEEEEEEEEEEEError getting dimensions: {why:?}"),
        }
    }
    commands.insert_resource(Tilesets {
        named_tilesets: my_map,
    });
}

pub fn init_serde_data(mut commands: Commands, serderes: Res<SERDEdata>) {
    let mut typeset = HashSet::new();
    let mut typehashmap: HashMap<String, Vec<Value>> = HashMap::new();
    let value_vector = &serderes.data;
    let lenny = &value_vector.len();
    let mut flagvec: Vec<String> = Vec::new();
    let mut itemmap: HashMap<String, Map<String, Value>> = HashMap::new();

    println!("serde length::::  {lenny}");

    for x in value_vector {
        let obj = &x.as_object().unwrap();

        match obj.get("type") {
            Some(id) => {
                typeset.insert(id.as_str().unwrap().to_string().to_lowercase());
            }

            None => {
                println!("{obj:#?}");
                panic!("no mitem typeeeeeeeeeeeee item count ");
            }
        } //end tagging match ritem.get("type"){
    }

    for jstype in &typeset {
        typehashmap.insert(jstype.to_string(), vec![]);
        //    println!("{jstype:#?}");
    }

    for y in value_vector {
        let obj = y.as_object().unwrap();

        let tyvm = obj.get("type").unwrap();

        let oby = tyvm.as_str().unwrap().to_string().to_lowercase();
        typehashmap.get_mut(&oby).unwrap().push(y.clone());
    }

    //TYPEHASHMAP is index by type and gives a vector of values of that type
    //everything begins here

    let item_types = vec![
        "armor".to_string(),
        "generic".to_string(),
        "gun".to_string(),
        "ammo".to_string(),
        "comestible".to_string(),
        "tool".to_string(),
        "tool_armor".to_string(),
        //      "monster".to_string(),
    ];
    for typed_vector in &typehashmap {
        let typed_vectortype = typed_vector.0.to_string();

        let iter_array = typed_vector.1;

        for obj in iter_array {
            if typed_vectortype == "json_flag" {
                println!("{obj:#?}");

                match obj.as_object().unwrap().get("id").unwrap() {
                    serde_json::Value::String(flag_string) => flagvec.push(flag_string.clone()),
                    _ => panic!("flag not strin {obj:?}"),
                }
            } else if item_types.contains(&typed_vectortype) {
                //
                println!("{obj:#?}");

                let whattheheck = match obj {
                    serde_json::Value::Object(item_map) => item_map,
                    _ => panic!("item not object"),
                };

                match whattheheck.get("abstract") {
                    Some(abid) => {
                        itemmap.insert(
                            abid.as_str().unwrap().to_string().clone(),
                            whattheheck.clone(),
                        );
                    }
                    None => match whattheheck.get("id") {
                        Some(abid) => match abid {
                            serde_json::Value::String(id_string) => {
                                itemmap.insert(id_string.clone(), whattheheck.clone());
                            }

                            serde_json::Value::Array(id_array) => {
                                for aid in id_array {
                                    itemmap.insert(
                                        aid.as_str().unwrap().to_string().clone(),
                                        whattheheck.clone(),
                                    );
                                }
                            }

                            _ => panic!(),
                        },

                        None => (), //no id or abstract
                    },
                }
            }
        }
    }
    println!("flagvec : {flagvec:#?}");
    println!("itemmap : {itemmap:#?}");

    let mut item_counter = 0;

    for item in &itemmap {
        item_counter += 1
    }

    let mut copy_looper = 1;
    let mut copy_loop_counter = 0;

    let mut itemmap_clone = itemmap.clone();

    let mut copy_from_counter = 0;
    let mut bad_counter = 0;

    while copy_looper > 0 {
        copy_looper = 0;
        copy_loop_counter += 1;

        for item in &mut itemmap {
            let fun_item = item.1.clone();
            match fun_item.get("copy-from") {
                //copy from inheritance
                Some(idtocopyfrom) => {
                    // println!("item is {:#?}",&item.1);

                    //   println!("SASDASDASd {:#?}",&idtocopyfrom);

                    match &itemmap_clone
                        .get(idtocopyfrom.as_str().unwrap())
                        .unwrap()
                        .get("copy-from")
                    {
                        Some(_) => {
                            println!("this is bad");
                            copy_looper += 1;
                            bad_counter += 1;
                        }
                        None => {
                            println!("this is good");
                            item.1.remove("copy-from");
                            copy_from_counter += 1;

                            let mut new_item_def = itemmap_clone
                                .get(idtocopyfrom.as_str().unwrap())
                                .unwrap()
                                .clone();

                            if let Some(x) = item.1.remove("extend") {
                                let extend_obj = x.as_object().unwrap();

                                for entry in extend_obj {
                                    let base_def = new_item_def.get_mut(entry.0);
                                    println!("z is {:?}", base_def);
                                    println!("entry is {:?}", entry);

                                    match base_def {
                                        None => {
                                            new_item_def.insert(entry.0.clone(), entry.1.clone());
                                        }
                                        Some(Value::Array(base_def_array)) => {
                                            let bd_value = entry.1.as_array().unwrap();
                                            base_def_array.append(&mut bd_value.clone());
                                        }
                                        Some(_) => panic!("def not an array: {:?}", base_def),
                                    }
                                }
                            }

                            if let Some(x) = item.1.remove("delete") {
                                let delete_obj = x.as_object().unwrap();

                                for entry in delete_obj {
                                    if let Some(Value::Array(base_def_array)) =
                                        new_item_def.get_mut(entry.0)
                                    {
                                        let bd_value = entry.1.as_array().unwrap();
                                        base_def_array.retain(|item| !bd_value.contains(item));
                                    } else if let Some(Value::String(_)) =
                                        new_item_def.get_mut(entry.0)
                                    {
                                        *new_item_def.get_mut(entry.0).unwrap() = Value::Null;
                                    } else {
                                        panic!("def not an array: {:?}", new_item_def.get(entry.0));
                                    }
                                }
                            }

                            if let Some(relative_value) = item.1.remove("relative") {
                                let relative_map = relative_value.as_object().unwrap();
                            
                                for (key, value) in relative_map {
                                    match new_item_def.get_mut(key) {
                                        None => {
                                            new_item_def.insert(key.clone(), value.clone());
                                        }
                                        Some(existing_value) => {
                                            match existing_value {
                                                Value::Number(existing_number) => {
                                                    *existing_value = serde_json::json!(
                                                        existing_number.as_f64().unwrap() + value.as_f64().unwrap()
                                                    );
                                                }
                                                Value::Object(existing_object) => {
                                                    if let Value::Object(value_object) = value {
                                                        for (inner_key, inner_value) in value_object {
                                                            match existing_object.get_mut(inner_key) {
                                                                None => {
                                                                    existing_object.insert(inner_key.clone(), inner_value.clone());
                                                                }
                                                                Some(existing_inner_value) => {
                                                                    if let Value::Number(existing_inner_number) = existing_inner_value {
                                                                        *existing_inner_value = serde_json::json!(
                                                                            existing_inner_number.as_f64().unwrap() + inner_value.as_f64().unwrap()
                                                                        );
                                                                    } else if existing_inner_value != inner_value {
                                                                        panic!("Mismatched value types: existing is {:?}, new is {:?}", existing_inner_value, inner_value);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        panic!("Expected an object, found {:?}", value);
                                                    }
                                                }
                                                _ => (), // Other types are not handled
                                            }
                                        }
                                    }
                                }
                            }
                            

                            match item.1.remove("proportional") {
                                _ => (),
                            }

                            match item.1.remove("replace_materials") {
                                _ => (),
                            }

                            for thing in &mut *item.1 {
                                //copies all new fields

                                new_item_def.insert(thing.0.clone(), thing.1.clone());
                            }
                            *item.1 = new_item_def; // this updates the item with its new definition after all inheritance processes are complete
                        }
                    }

                    //    let x: i32 = idtocopyfrom;
                }

                None => (),
            }
        }

        itemmap_clone = itemmap.clone();
        println!("COPY LOOP BREAK");
    }

    println!("copy from counter: {copy_from_counter:?}");
    println!("bad bad counter: {bad_counter:?}");
    println!("copyloop counter: {copy_loop_counter:?}");
    println!("item counter: {item_counter:?}");
    println!(
        "reloaded_9mmP is now::   {:?}",
        itemmap.get("reloaded_9mmP")
    );

    commands.insert_resource(SmartData {
        item_map: itemmap,
        json_flag_set: flagvec,
    });
}
