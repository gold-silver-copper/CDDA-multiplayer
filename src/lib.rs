mod components;

mod system_helpers;

mod init_helpers;

pub mod systems;
pub use systems::*;

pub struct CCCPlugin;

impl Plugin for CCCPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Position>()
            .replicate::<Player>()
            .replicate::<PhysicalCharacteristics>()
            .replicate::<WieldedItems>()
            .replicate::<WornItems>()
            //  .replicate::<Transform>()
            //    .replicate::<Visibility>()
            //   .replicate::<GlobalTransform>()
            .add_client_event::<MoveDirection>(EventType::Ordered)
            .add_systems(PreStartup, (setup, init_tilesets, init_json2))
            .add_systems(Startup, (init_sprite_bundles, init_serde_data)) // , init_serde_data
            .add_systems(
                PostStartup,
                (Self::cli_system.map(Result::unwrap), spawn_test_map),
            )
            .add_systems(
                Update,
                (
                    attach_local_player_component.run_if(resource_exists::<LocalPlayerResource>()),
                    movement_system.run_if(has_authority()), // Runs only on the server or a single player.
                    Self::server_event_system.run_if(resource_exists::<RenetServer>()), // Runs only on the server.
                    attach_clothes,
                    spawn_items,
                ),
            )
            .add_systems(Update, input_system)
            .add_systems(Update, ui_example_system)
            .add_systems(Update, update_transforms)
            .add_systems(PostUpdate, update_sprites)
            .add_systems(PostUpdate, update_camera);
    }
}

impl CCCPlugin {
    fn cli_system(
        mut commands: Commands,
        cli: Res<Cli>,
        network_channels: Res<NetworkChannels>,
    ) -> Result<(), Box<dyn Error>> {
        match *cli {
            Cli::SinglePlayer => {
                let local_player = LocalPlayerResource { id: SERVER_ID };
                commands.insert_resource(local_player);
                commands.spawn((
                    CatCharBundle {
                        player: Player(SERVER_ID),
                        ..Default::default()
                    },
                    LocalPlayerComponent {
                        id: ClientId::from_raw(0),
                    },
                ));
            }
            Cli::Server { port } => {
                let server_channels_config = network_channels.get_server_configs();
                let client_channels_config = network_channels.get_client_configs();

                let server = RenetServer::new(ConnectionConfig {
                    server_channels_config,
                    client_channels_config,
                    ..Default::default()
                });

                let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
                let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
                let socket = UdpSocket::bind(public_addr)?;
                let server_config = ServerConfig {
                    current_time,
                    max_clients: 10,
                    protocol_id: PROTOCOL_ID,
                    authentication: ServerAuthentication::Unsecure,
                    public_addresses: vec![public_addr],
                };
                let transport = NetcodeServerTransport::new(server_config, socket)?;

                commands.insert_resource(server);
                commands.insert_resource(transport);

                let local_player = LocalPlayerResource { id: SERVER_ID };

                commands.insert_resource(local_player);

                commands.spawn((
                    CatCharBundle {
                        player: Player(SERVER_ID),
                        ..Default::default()
                    },
                    LocalPlayerComponent {
                        id: ClientId::from_raw(0),
                    },
                ));
            }
            Cli::Client { port, ip } => {
                let server_channels_config = network_channels.get_server_configs();
                let client_channels_config = network_channels.get_client_configs();

                let client = RenetClient::new(ConnectionConfig {
                    server_channels_config,
                    client_channels_config,
                    ..Default::default()
                });

                let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
                let client_id = current_time.as_millis() as u64;
                let server_addr = SocketAddr::new(ip, port);
                let socket = UdpSocket::bind((ip, 0))?;
                let authentication = ClientAuthentication::Unsecure {
                    client_id,
                    protocol_id: PROTOCOL_ID,
                    server_addr,
                    user_data: None,
                };
                let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

                commands.insert_resource(client);
                commands.insert_resource(transport);
                let local_player = LocalPlayerResource {
                    id: ClientId::from_raw(client_id),
                };
                commands.insert_resource(local_player);
            }
        }

        Ok(())
    }

    /// Logs server events and spawns a new player whenever a client connects.
    fn server_event_system(mut commands: Commands, mut server_event: EventReader<ServerEvent>) {
        // println!("SERVER EVENT MEOW");
        for event in server_event.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("player: {client_id} Connected");

                    commands.spawn((CatCharBundle {
                        player: Player(*client_id),
                        ..Default::default()
                    },));
                }

                ServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("client {client_id} disconnected: {reason}");
                }
            }
        }
    }
}

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 0;

#[derive(Parser, PartialEq, Resource)]
pub enum Cli {
    SinglePlayer,
    Server {
        #[arg(short, long, default_value_t = PORT)]
        port: u16,
    },
    Client {
        #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST.into())]
        ip: IpAddr,

        #[arg(short, long, default_value_t = PORT)]
        port: u16,
    },
}

impl Default for Cli {
    fn default() -> Self {
        Cli::SinglePlayer // Self::parse()
    }
}
