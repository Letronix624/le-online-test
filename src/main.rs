use std::sync::Arc;

use let_engine::prelude::*;

use tracing::info;

#[cfg(feature = "client")]
fn main() {
    tracing_subscriber::fmt::init();

    // First you make a builder containing the description of the window.
    let window_builder = WindowBuilder::new().inner_size(vec2(1280.0, 720.0));
    // Then you start the engine allowing you to load resources and layers.
    let mut engine = Engine::<Game, ()>::new(
        EngineSettingsBuilder::default()
            .window_settings(window_builder)
            .build()
            .unwrap(),
    )
    .unwrap();

    let server = engine.new_public_server(7777).unwrap();
    server.start();

    // Here it initializes the game struct to be used with the engine run method.
    let game = Game::new();

    // Runs the game engine and makes a window.
    engine.start(game);
}

/// Makes a game struct containing
struct Game {
    /// the main layer, where the scene gets put inside,
    main_layer: Arc<Layer>,
    circle: Option<Object>,
    /// a variable that decides whether the program should close.
    exit: bool,
}

impl Game {
    /// Constructor for this scene.
    pub fn new() -> Self {
        Self {
            // Makes a base layer where you place your scene into.
            main_layer: SCENE.new_layer(),
            circle: None,
            exit: false,
        }
    }
}

/// Implement the Game trait into the Game struct.
impl let_engine::Game<()> for Game {
    async fn start(&mut self) {
        // Makes the view zoomed out and not stretchy.
        self.main_layer.set_camera_settings(CameraSettings {
            zoom: 0.5,
            mode: CameraScaling::Expand,
        });
        // Makes the circle in the middle.
        let mut circle = NewObject::new();
        // Loads a circle model into the engine and sets the appearance of this object to it.
        circle
            .appearance
            .set_model(Some(Model::Custom(
                ModelData::new(make_circle!(30)).unwrap(),
            )))
            .unwrap();
        // Initializes the object to the layer
        self.circle = circle.init(&self.main_layer).ok();
    }
    #[cfg(feature = "client")]
    async fn event(&mut self, event: Event) {
        match event {
            // Exit when the X button is pressed.
            Event::Window(WindowEvent::CloseRequested) => {
                self.exit = true;
            }
            Event::Input(InputEvent::KeyboardInput { input }) => {
                if input.state == ElementState::Pressed {
                    if let Key::Named(NamedKey::Escape) = input.key {
                        // Exit when the escape key is pressed.
                        self.exit = true;
                    }
                }
            }
            _ => (),
        };
    }

    async fn net_event(
        &mut self,
        addr: std::net::SocketAddr,
        message: networking::RemoteMessage<()>,
    ) {
        match message {
            networking::RemoteMessage::Connected => {
                let Some(circle) = self.circle.as_mut() else {
                    return;
                };

                circle.update().unwrap();
                circle.appearance.set_color(Color::GREEN);
                circle.sync().unwrap();

                info!("{addr}: Connected");
            }
            networking::RemoteMessage::Disconnected => {
                let Some(circle) = self.circle.as_mut() else {
                    return;
                };

                circle.update().unwrap();
                circle.appearance.set_color(Color::RED);
                circle.sync().unwrap();

                info!("{addr}: Disconnected");
            }
            _ => {
                let Some(circle) = self.circle.as_mut() else {
                    return;
                };

                circle.update().unwrap();
                circle.appearance.set_color(Color::BLUE);
                circle.sync().unwrap();

                info!("{addr}: Sent a package");
            }
        }
    }

    /// Exits the program in case `self.exit` is true.
    fn exit(&self) -> bool {
        self.exit
    }
}
