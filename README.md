# VOID RUNNER

it is a browser-based survival game built to practice game development, Rust backend development, websockets, real-time state management, and client-server architecture.

## Decription
This is a simple survival game where the player controls a character, collects energy, & avoid enemy that continus chase the player  this game build on html, css, javascript, rust, axum and websocket. i built the frontend using html css & javascript. and game login and server side state are handle by rust.
it is a browser-based survival game built to practice game development you have to douch the enemy and collect the food, and you will get different power such as 2x speed, 2x score, shifty shield for protection, it has light, dark and neon theme which works perfect, has a menu option in sidebar for controlling all feature of the game. that increase your points, Rust backend development, websockets, real-time state management, and client-server architecture.

the rust backend is responsible for:
- player movement
- enemy spawing 
- enemy movement
- enemy movement 
- collision detection
- energy collection
- score calculation
- difficulty scaling 
- game state
- websocket communication
- player sessions

the frontend is mainly responsiable for rendering the game and sending player input to the rust. the current architectuere uses a `GameServer` containing multiple player sessions:

```text
Game server -> playerId 1= game session
                playerid 2 = game session
                playerid 3= game session

## dependency
cargo add axum --features ws
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add rand
cargo add futures-util

## Get started:
git clone https://github.com/rajayadavikrahi/space-game
cd space-game
cd client && python3 -m http.server 5500
cd .. && cd rust && cargo run


## License
This project is currently a game project. No specific open-source license has been added yet.
