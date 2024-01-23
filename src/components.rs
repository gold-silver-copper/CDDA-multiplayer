pub use bevy::prelude::*;
pub use bevy::utils::HashMap;
pub use bevy::utils::HashSet;
pub use bevy_egui::{egui, EguiContexts, EguiPlugin};
pub use imagesize::*;
pub use serde_json::Map;
pub use serde_json::Value;
pub use std::fs;
pub use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

pub use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{
            ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport,
            ServerAuthentication, ServerConfig,
        },
        ClientId, ConnectionConfig, ServerEvent,
    },
};

pub use clap::Parser;
pub use ordered_float::OrderedFloat;
pub use serde::{Deserialize, Serialize};
pub use walkdir::WalkDir;

/// Contains the client ID of the player.
#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub ClientId);

#[derive(Component)] //, Deref, DerefMut, Debug
pub struct LocalPlayerComponent {
    pub id: ClientId,
}

#[derive(Component, Serialize, Deserialize)] //, Deref, DerefMut, Debug
pub struct Initialized {}

#[derive(Component, Serialize, Deserialize)] //, Deref, DerefMut, Debug
pub struct WieldedItems {
    pub wielded: Vec<String>, //key = order put on
}
#[derive(Component, Serialize, Deserialize)] //, Deref, DerefMut, Debug
pub struct WornItems {
    pub worn: Vec<String>, //key = order put on
}

#[derive(Component, Serialize, Deserialize)] //, Deref, DerefMut, Debug
pub struct PhysicalCharacteristics {
    pub gender: String,
    pub eye_color: String,
    pub hair_color: String,
    pub hair_style: String,
    pub skin: String,
}

impl Default for PhysicalCharacteristics {
    fn default() -> PhysicalCharacteristics {
        PhysicalCharacteristics {
            gender: "male".to_string(),
            hair_color: "brown".to_string(),
            hair_style: "short".to_string(),
            eye_color: "gray".to_string(),
            skin: "pink".to_string(),
        }
    }
}

#[derive(Component, Serialize, Deserialize)] //, Deref, DerefMut, Debug
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)] //, Deref, DerefMut, Debug
pub struct PreviousGlobalTransform {
    pub translation: Vec3,
}
impl Default for PreviousGlobalTransform {
    fn default() -> PreviousGlobalTransform {
        PreviousGlobalTransform {
            translation: Vec3::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct ClothingSpriteBundle {
    pub spritesheetbundle: SpriteSheetBundle,
    pub prevglobtrans: PreviousGlobalTransform,
}

#[derive(Component)] //, Deref, DerefMut, Debug
pub struct ItemDetails {
    pub item_id: String,
    //  pub item_name: String,
    //  pub item_name_pl: String,
    //   pub item_volume: f32, // in ml
    //  pub item_weight: f32, // in grams
}

#[derive(Bundle)]
pub struct CatItemBundle {
    // A bundle can contain components
    pub pos: Position,
    pub replication: Replication,
    pub item_details: ItemDetails,
    pub spatial_bundle: SpatialBundle,
}

impl Default for CatItemBundle {
    fn default() -> CatItemBundle {
        CatItemBundle {
            pos: Position { x: 0, y: 0 },

            replication: Replication,
            item_details: ItemDetails {
                item_id: "borkborkborkd".to_string(),
            },
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, 0.0, 0.0,
            ))),
        }
    }
}

#[derive(Bundle)]
pub struct CatCharBundle {
    // A bundle can contain components
    pub pos: Position,
    pub physical: PhysicalCharacteristics,
    pub worn: WornItems,
    pub wielded: WieldedItems,
    pub replication: Replication,
    pub player: Player,
}

impl Default for CatCharBundle {
    fn default() -> CatCharBundle {
        CatCharBundle {
            pos: Position { x: 5, y: 1 },
            physical: PhysicalCharacteristics { ..default() },
            worn: WornItems {
                worn: vec![
                    "garter_belt".to_string(), //"kippah".to_string(),
                ],
            },
            wielded: WieldedItems {
                wielded: vec!["fn_fal".to_string()],
            },
            replication: Replication,
            player: Player(ClientId::from_raw(111111)),
        }
    }
}

/// A movement event for the controlled box.
#[derive(Debug, PartialEq, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug)]
pub struct SpriteInfo {
    pub sprite_id: String,

    pub fg: i32,
    pub bg: i32,
    pub rotates: bool,
    pub multitile: bool,
}

impl Default for SpriteInfo {
    fn default() -> SpriteInfo {
        SpriteInfo {
            sprite_id: "DEFAULTID".to_string(),
            fg: -1,
            bg: -1,
            rotates: false,
            multitile: false,
        }
    }
}

#[derive(Debug)]
pub struct TileIndex {
    pub tileset_name: String,
    pub tileset_index: i32,
}

impl Default for TileIndex {
    fn default() -> TileIndex {
        TileIndex {
            tileset_name: "tiles".to_string(),
            tileset_index: 0,
        }
    }
}

impl MoveDirection {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }
}

#[derive(Resource)]
pub struct Tilesets {
    pub named_tilesets: HashMap<String, Handle<TextureAtlas>>,
}

pub struct ItemToSpawn {
    pub item_id: String,
    pub spawn_pos: Position,
}

#[derive(Resource)]
pub struct ItemSpawnQueue {
    pub item_spawn_queue: Vec<ItemToSpawn>,
}

impl Default for ItemSpawnQueue {
    fn default() -> ItemSpawnQueue {
        ItemSpawnQueue {
            item_spawn_queue: vec![],
        }
    }
}

#[derive(Resource)]
pub struct SpriteInfoMap {
    pub simap: HashMap<String, SpriteInfo>,
}

#[derive(Resource)]
pub struct TileIndexMap {
    pub timap: HashMap<i32, TileIndex>,
}

#[derive(Resource)]
pub struct LocalPlayerResource {
    pub id: ClientId,
}

#[derive(Resource)]
pub struct SERDEdata {
    pub data: Vec<Value>,
}

#[derive(Resource)]
pub struct SmartData {
    pub item_map: HashMap<String, Map<String, Value>>,
    pub json_flag_set: Vec<String>,
}
