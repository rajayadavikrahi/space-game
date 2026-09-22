const canvas =
    document.getElementById(
        "gameCanvas"
    );

const ctx =
    canvas.getContext("2d");

const scoreElement =
    document.getElementById(
        "score"
    );

const startScreen =
    document.getElementById(
        "startScreen"
    );

const gameOverScreen =
    document.getElementById(
        "gameOverScreen"
    );

const startButton =
    document.getElementById(
        "startButton"
    );

const restartButton =
    document.getElementById(
        "restartButton"
    );

const finalScoreElement =
    document.getElementById(
        "finalScore"
    );


// ==========================================
// WEBSOCKET
// ==========================================

const WS_URL =
    "ws://127.0.0.1:3000/ws";

let socket = null;

let gameState = null;

let playerId = null;


// ==========================================
// INPUT
// ==========================================

let keys = {
    up: false,

    down: false,

    left: false,

    right: false,
};


// ==========================================
// KEY DOWN
// ==========================================

window.addEventListener(
    "keydown",
    (event) => {

        switch (
            event.key.toLowerCase()
        ) {

            case "w":
            case "arrowup":

                keys.up = true;

                event.preventDefault();

                break;


            case "s":
            case "arrowdown":

                keys.down = true;

                event.preventDefault();

                break;


            case "a":
            case "arrowleft":

                keys.left = true;

                event.preventDefault();

                break;


            case "d":
            case "arrowright":

                keys.right = true;

                event.preventDefault();

                break;
        }

        sendInput();
    }
);


// ==========================================
// KEY UP
// ==========================================

window.addEventListener(
    "keyup",
    (event) => {

        switch (
            event.key.toLowerCase()
        ) {

            case "w":
            case "arrowup":

                keys.up = false;

                break;


            case "s":
            case "arrowdown":

                keys.down = false;

                break;


            case "a":
            case "arrowleft":

                keys.left = false;

                break;


            case "d":
            case "arrowright":

                keys.right = false;

                break;
        }

        sendInput();
    }
);


// ==========================================
// CONNECT
// ==========================================

function connectWebSocket() {

    socket =
        new WebSocket(
            WS_URL
        );


    socket.addEventListener(
        "open",
        () => {

            console.log(
                "Connected to Rust server"
            );

            sendInput();
        }
    );


    socket.addEventListener(
        "message",
        (event) => {

            try {

                const message =
                    JSON.parse(
                        event.data
                    );


                // ==========================
                // PLAYER CONNECTED
                // ==========================

                if (
                    message.type ===
                    "connected"
                ) {

                    playerId =
                        message.player_id;

                    console.log(
                        "Player ID:",
                        playerId
                    );

                    return;
                }


                // ==========================
                // GAME STATE
                // ==========================

                if (
                    message.type ===
                    "state"
                ) {

                    gameState =
                        message.game;

                    updateHUD();

                    handleGameState();

                    render();
                }

            } catch (error) {

                console.error(
                    "Invalid server message:",
                    error
                );
            }
        }
    );


    socket.addEventListener(
        "close",
        () => {

            console.log(
                "Disconnected from Rust server"
            );
        }
    );


    socket.addEventListener(
        "error",
        (error) => {

            console.error(
                "WebSocket error:",
                error
            );
        }
    );
}


// ==========================================
// SEND INPUT
// ==========================================

function sendInput() {

    if (
        !socket ||
        socket.readyState !==
            WebSocket.OPEN
    ) {
        return;
    }


    let directionX = 0;

    let directionY = 0;


    if (keys.left) {
        directionX -= 1;
    }

    if (keys.right) {
        directionX += 1;
    }

    if (keys.up) {
        directionY -= 1;
    }

    if (keys.down) {
        directionY += 1;
    }


    socket.send(
        JSON.stringify({
            type: "input",

            direction_x:
                directionX,

            direction_y:
                directionY,
        })
    );
}


// ==========================================
// START GAME
// ==========================================

function startGame() {

    if (
        !socket ||
        socket.readyState !==
            WebSocket.OPEN
    ) {

        alert(
            "Not connected to Rust server."
        );

        return;
    }


    socket.send(
        JSON.stringify({
            type: "start",
        })
    );


    startScreen.classList.add(
        "hidden"
    );

    gameOverScreen.classList.add(
        "hidden"
    );
}


// ==========================================
// RESET GAME
// ==========================================

function resetGame() {

    if (
        !socket ||
        socket.readyState !==
            WebSocket.OPEN
    ) {

        alert(
            "Not connected to Rust server."
        );

        return;
    }


    socket.send(
        JSON.stringify({
            type: "reset",
        })
    );


    gameOverScreen.classList.add(
        "hidden"
    );

    startScreen.classList.remove(
        "hidden"
    );
}


// ==========================================
// HUD
// ==========================================

function updateHUD() {

    if (!gameState) {
        return;
    }

    scoreElement.textContent =
        gameState.score;
}


// ==========================================
// GAME STATE
// ==========================================

function handleGameState() {

    if (!gameState) {
        return;
    }


    if (
        gameState.status ===
        "game_over"
    ) {

        handleGameOver();
    }
}


// ==========================================
// GAME OVER
// ==========================================

function handleGameOver() {

    finalScoreElement.textContent =
        gameState.score;

    gameOverScreen.classList.remove(
        "hidden"
    );
}


// ==========================================
// RENDER
// ==========================================

function render() {

    if (!gameState) {
        return;
    }


    ctx.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
    );


    drawBackground();

    drawEnergy();

    drawEnemies();

    drawPlayer();
}


// ==========================================
// BACKGROUND
// ==========================================

function drawBackground() {

    ctx.fillStyle =
        "#050505";

    ctx.fillRect(
        0,
        0,
        canvas.width,
        canvas.height
    );


    drawGrid();
}


// ==========================================
// GRID
// ==========================================

function drawGrid() {

    const gridSize = 40;


    ctx.strokeStyle =
        "#111";

    ctx.lineWidth = 1;


    for (
        let x = 0;

        x <= canvas.width;

        x += gridSize
    ) {

        ctx.beginPath();

        ctx.moveTo(
            x,
            0
        );

        ctx.lineTo(
            x,
            canvas.height
        );

        ctx.stroke();
    }


    for (
        let y = 0;

        y <= canvas.height;

        y += gridSize
    ) {

        ctx.beginPath();

        ctx.moveTo(
            0,
            y
        );

        ctx.lineTo(
            canvas.width,
            y
        );

        ctx.stroke();
    }
}


// ==========================================
// PLAYER
// ==========================================

function drawPlayer() {

    const player =
        gameState.player;


    ctx.beginPath();


    ctx.arc(
        player.x,
        player.y,
        player.size,
        0,
        Math.PI * 2
    );


    ctx.fillStyle =
        "#ffffff";


    ctx.fill();

    ctx.closePath();
}


// ==========================================
// ENERGY
// ==========================================

function drawEnergy() {

    const energy =
        gameState.energy;


    ctx.beginPath();


    ctx.arc(
        energy.x,
        energy.y,
        energy.size,
        0,
        Math.PI * 2
    );


    ctx.fillStyle =
        "#00ff88";


    ctx.shadowBlur = 20;

    ctx.shadowColor =
        "#00ff88";


    ctx.fill();


    ctx.shadowBlur = 0;


    ctx.closePath();
}


// ==========================================
// ENEMIES
// ==========================================

function drawEnemies() {

    for (
        const enemy
        of gameState.enemies
    ) {

        ctx.beginPath();


        ctx.arc(
            enemy.x,
            enemy.y,
            enemy.size,
            0,
            Math.PI * 2
        );


        ctx.fillStyle =
            "#ff3355";


        ctx.shadowBlur = 15;

        ctx.shadowColor =
            "#ff3355";


        ctx.fill();


        ctx.shadowBlur = 0;


        ctx.closePath();
    }
}


// ==========================================
// BUTTONS
// ==========================================

startButton.addEventListener(
    "click",
    () => {

        startGame();
    }
);


restartButton.addEventListener(
    "click",
    () => {

        resetGame();
    }
);


// ==========================================
// INITIALIZE
// ==========================================

function initialize() {

    connectWebSocket();
}


initialize();