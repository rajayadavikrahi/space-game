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

const gameTimeElement =
    document.getElementById(
        "gameTime"
    );

const gameLevelElement =
    document.getElementById(
        "gameLevel"
    );

const powerupStatusElement =
    document.getElementById(
        "powerupStatus"
    );

const sidebarScoreElement =
    document.getElementById(
        "sidebarScore"
    );

const highScoreElement =
    document.getElementById(
        "highScore"
    );

const gamesPlayedElement =
    document.getElementById(
        "gamesPlayed"
    );

const totalTimeElement =
    document.getElementById(
        "totalTime"
    );

const startScreen =
    document.getElementById(
        "startScreen"
    );

const gameOverScreen =
    document.getElementById(
        "gameOverScreen"
    );

const pauseScreen =
    document.getElementById(
        "pauseScreen"
    );

const startButton =
    document.getElementById(
        "startButton"
    );

const restartButton =
    document.getElementById(
        "restartButton"
    );

const resumeButton =
    document.getElementById(
        "resumeButton"
    );

const quitButton =
    document.getElementById(
        "quitButton"
    );

const finalScoreElement =
    document.getElementById(
        "finalScore"
    );

const finalTimeElement =
    document.getElementById(
        "finalTime"
    );

const finalHighScoreElement =
    document.getElementById(
        "finalHighScore"
    );

const sidebar =
    document.getElementById(
        "sidebar"
    );

const toggleSidebarButton =
    document.getElementById(
        "toggleSidebar"
    );

const soundToggle =
    document.getElementById(
        "soundToggle"
    );

const musicToggle =
    document.getElementById(
        "musicToggle"
    );

const difficultySelect =
    document.getElementById(
        "difficultySelect"
    );

const themeSelect =
    document.getElementById(
        "themeSelect"
    );

const resetStatsButton =
    document.getElementById(
        "resetStatsButton"
    );

const WS_URL =
    window.VOIDRUNNER_BACKEND_URL ||
    "ws://127.0.0.1:3000/ws";

let socket = null;

let gameState = null;

let previousGameStatus = null;

let playerId = null;

let isReconnecting = false;

let keys = {
    up: false,
    down: false,
    left: false,
    right: false,
};

// Player ship facing (radians), sprite points up
let playerFacing = 0;

// Game statistics
let gameStats = {
    highScore: parseInt(localStorage.getItem('highScore')) || 0,
    gamesPlayed: parseInt(localStorage.getItem('gamesPlayed')) || 0,
    totalTime: parseInt(localStorage.getItem('totalTime')) || 0,
    currentGameTime: 0,
    lastScore: 0,
    gameStartTime: null,
    pauseStartTime: null,
    totalPausedTime: 0,
    isPaused: false
};

// Settings
let settings = {
    soundEnabled: true,
    musicEnabled: true,
    difficulty: 'normal',
    theme: 'dark'
};

// Audio context for sound effects
let audioContext = null;

// Background music nodes
let musicNodes = null;


// ============================================
// KEYBOARD INPUT
// ============================================

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
                
            case "p":
                togglePause();
                event.preventDefault();
                break;
        }

        sendInput();
    }
);


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


// ============================================
// WEBSOCKET
// ============================================

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
            
            isReconnecting = false;

            sendInput();
            
            // Re-apply the selected difficulty to a fresh session
            if (socket && socket.readyState === WebSocket.OPEN) {
                socket.send(JSON.stringify({
                    type: "difficulty",
                    difficulty: settings.difficulty
                }));
            }
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
            
            // Attempt to reconnect after 3 seconds if not already reconnecting
            if (!isReconnecting) {
                isReconnecting = true;
                setTimeout(() => {
                    console.log("Attempting to reconnect...");
                    connectWebSocket();
                }, 3000);
            }
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


// ============================================
// INPUT TO SERVER
// ============================================

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

    if (directionX !== 0 || directionY !== 0) {
        playerFacing = Math.atan2(directionX, -directionY);
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


// ============================================
// GAME CONTROLS
// ============================================

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

    // Reset current game time
    gameStats.currentGameTime = 0;
    gameStats.lastScore = 0;
    gameStats.gameStartTime = Date.now();
    gameStats.pauseStartTime = null;
    gameStats.totalPausedTime = 0;
    gameStats.isPaused = false;

    socket.send(
        JSON.stringify({
            type: "start",
        })
    );

    if (startScreen) startScreen.classList.add("hidden");
    if (gameOverScreen) gameOverScreen.classList.add("hidden");
    if (pauseScreen) pauseScreen.classList.add("hidden");
}


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

    if (gameOverScreen) gameOverScreen.classList.add("hidden");
    if (pauseScreen) pauseScreen.classList.add("hidden");
    if (startScreen) startScreen.classList.remove("hidden");
}


// ============================================
// HUD
// ============================================

function updateHUD() {
    if (!gameState) {
        return;
    }

    if (scoreElement) scoreElement.textContent = gameState.score;
    if (sidebarScoreElement) sidebarScoreElement.textContent = gameState.score;
    
    // Play a collect sound when the score goes up
    if (gameState.score > gameStats.lastScore) {
        playSound('collect');
        gameStats.lastScore = gameState.score;
    }
    
    // Update game time if game is running
    if (gameState.status === "running" && !gameStats.isPaused && gameStats.gameStartTime) {
        const elapsedMilliseconds = Date.now() - gameStats.gameStartTime - gameStats.totalPausedTime;
        const elapsedSeconds = Math.floor(elapsedMilliseconds / 1000);
        gameStats.currentGameTime = elapsedSeconds;
        if (gameTimeElement) gameTimeElement.textContent = formatTime(gameStats.currentGameTime);
    }
    
    // Update level (basic implementation based on score)
    const level = Math.floor(gameState.score / 100) + 1;
    if (gameLevelElement) gameLevelElement.textContent = level;
    
    // Keep sidebar statistics live
    if (gameState.score > gameStats.highScore) {
        gameStats.highScore = gameState.score;
        saveStatistics();
    }
    if (highScoreElement) highScoreElement.textContent = gameStats.highScore;
    if (gamesPlayedElement) gamesPlayedElement.textContent = gameStats.gamesPlayed;
    if (totalTimeElement) totalTimeElement.textContent = formatTime(gameStats.totalTime + gameStats.currentGameTime);
    
    // Update power-up status
    updatePowerUpStatus();
}


function handleGameState() {
    if (!gameState) {
        return;
    }

    const status = gameState.status;

    // Count game over only on the transition into
    // game_over. The server keeps streaming
    // "game_over" ticks at 60fps, so this guard
    // prevents handleGameOver from running (and
    // inflating games played) on every tick.
    if (status === "game_over" && previousGameStatus !== "game_over") {
        handleGameOver();
    }

    previousGameStatus = status;
}

function updatePowerUpStatus() {
    if (!powerupStatusElement) return;
    
    const activePowerup = gameState && gameState.active_powerup;
    
    if (activePowerup) {
        const label = activePowerup === "double_score" ? "2X" : activePowerup.toUpperCase();
        powerupStatusElement.textContent = `${label} ACTIVE`;
        powerupStatusElement.classList.remove("empty");
        powerupStatusElement.classList.add("active");
        return;
    }
    
    const nearby = (gameState && gameState.powerups && Array.isArray(gameState.powerups))
        ? gameState.powerups.length
        : 0;
    
    if (nearby > 0) {
        powerupStatusElement.textContent = `${nearby} NEARBY`;
        powerupStatusElement.classList.remove("empty");
        powerupStatusElement.classList.add("active");
    } else {
        powerupStatusElement.textContent = "NONE";
        powerupStatusElement.classList.remove("active");
        powerupStatusElement.classList.add("empty");
    }
}


function handleGameOver() {
    if (finalScoreElement) finalScoreElement.textContent = gameState.score;
    if (finalTimeElement) finalTimeElement.textContent = formatTime(gameStats.currentGameTime);
    
    // Update statistics
    if (gameState.score > gameStats.highScore) {
        gameStats.highScore = gameState.score;
        playSound('powerup'); // Play sound for new high score
    }
    
    // Only increment games played if the game actually ran for some time
    // This prevents counting quick restarts as full games
    if (gameStats.currentGameTime > 0) {
        gameStats.gamesPlayed++;
    }
    
    gameStats.totalTime += gameStats.currentGameTime;
    
    if (finalHighScoreElement) finalHighScoreElement.textContent = gameStats.highScore;
    
    saveStatistics();
    updateStatistics();
    
    playSound('gameover');

    if (gameOverScreen) gameOverScreen.classList.remove("hidden");
}


// ============================================
// SVG SPRITES
// ============================================

const sprites = {};

function loadSprite(name, svg) {
    const image = new Image();
    image.src = "data:image/svg+xml;charset=utf-8," + encodeURIComponent(svg);
    image.addEventListener("load", () => {
        image.ready = true;
    });
    sprites[name] = image;
    return image;
}

loadSprite("player", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <defs>
    <linearGradient id="hull" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#ffffff"/>
      <stop offset="1" stop-color="#8fd9ff"/>
    </linearGradient>
  </defs>
  <path d="M32 4 L48 46 L32 36 L16 46 Z" fill="url(#hull)" stroke="#27738c" stroke-width="2" stroke-linejoin="round"/>
  <path d="M32 4 L40 33 L32 36 L24 33 Z" fill="#eaf9ff" opacity="0.8"/>
  <circle cx="32" cy="23" r="5" fill="#00ff88" stroke="#007a45" stroke-width="1.5"/>
  <ellipse cx="46" cy="50" rx="6" ry="3" fill="#ff3355"/>
  <ellipse cx="18" cy="50" rx="6" ry="3" fill="#ff3355"/>
</svg>`);

loadSprite("enemy", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <path d="M32 6 L40 24 L58 28 L46 42 L50 58 L32 50 L14 58 L18 42 L6 28 L24 24 Z" fill="#ff3355" stroke="#8f1d31" stroke-width="2" stroke-linejoin="round"/>
  <circle cx="25" cy="34" r="5" fill="#21060c"/>
  <circle cx="39" cy="34" r="5" fill="#21060c"/>
  <circle cx="26.5" cy="32.5" r="1.8" fill="#fff"/>
  <circle cx="40.5" cy="32.5" r="1.8" fill="#fff"/>
</svg>`);

loadSprite("energy", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="29" fill="#00ff88" opacity="0.18"/>
  <path d="M32 6 L54 32 L32 58 L10 32 Z" fill="#00ff88" stroke="#00cc6a" stroke-width="2" stroke-linejoin="round"/>
  <path d="M32 6 L43 32 L32 34 Z" fill="#c9ffea"/>
  <path d="M32 34 L43 32 L32 58 Z" fill="#00b35c" opacity="0.55"/>
</svg>`);

loadSprite("shield", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#0b1a33" stroke="#4488ff" stroke-width="3"/>
  <path d="M32 13 L47 19 V34 C47 43 41 50 32 53 C23 50 17 43 17 34 V19 Z" fill="#4488ff" stroke="#9cc7ff" stroke-width="1.5"/>
  <text x="32" y="39" text-anchor="middle" font-family="Arial, sans-serif" font-size="18" font-weight="bold" fill="#eaf2ff">S</text>
</svg>`);

loadSprite("speed", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#3a2500" stroke="#ffaa00" stroke-width="3"/>
  <path d="M36 10 L18 37 H29 L25 54 L46 25 H35 Z" fill="#ffaa00" stroke="#ffe08a" stroke-width="1.5" stroke-linejoin="round"/>
</svg>`);

loadSprite("double_score", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#2a0f3d" stroke="#cc55ff" stroke-width="3"/>
  <path d="M32 8 L38 24 L55 27 L42 38 L46 55 L32 46 L18 55 L22 38 L9 27 L26 24 Z" fill="#cc55ff" stroke="#e3b3ff" stroke-width="1.2" stroke-linejoin="round"/>
  <text x="32" y="39" text-anchor="middle" font-family="Arial, sans-serif" font-size="15" font-weight="bold" fill="#ffffff">2X</text>
</svg>`);


// Draw a loaded SVG sprite centered at (x, y).
// Returns true when drawn, false if the sprite
// isn't loaded yet (caller falls back to shapes).
function drawSprite(name, x, y, width, height, angle) {
    const image = sprites[name];

    if (!image || !image.ready) {
        return false;
    }

    ctx.save();

    ctx.translate(x, y);

    if (angle) {
        ctx.rotate(angle);
    }

    ctx.drawImage(
        image,
        -width / 2,
        -height / 2,
        width,
        height
    );

    ctx.restore();

    return true;
}


// Gentle pulsing scale factor for collectibles.
function spritePulse() {
    const now =
        typeof performance !== "undefined"
            ? performance.now()
            : Date.now();

    return 1 + 0.12 * Math.sin(now / 180);
}


// ============================================
// MAIN RENDER FUNCTION
// ============================================

function render() {
    if (!gameState || !canvas || !ctx) {
        return;
    }

    ctx.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
    );

    drawBackground();

    if (gameState.energy) {
        drawEnergy();
    }

    if (gameState.powerups && Array.isArray(gameState.powerups)) {
        drawPowerUps();
    }

    if (gameState.enemies && Array.isArray(gameState.enemies)) {
        drawEnemies();
    }

    if (gameState.player) {
        drawPlayer();
    }
}


// ============================================
// BACKGROUND
// ============================================

function drawBackground() {
    const themeColors = {
        dark: "#050505",
        light: "#dcdcdc",
        neon: "#0b0b1a"
    };
    
    ctx.fillStyle =
        themeColors[settings.theme] || "#050505";

    ctx.fillRect(
        0,
        0,
        canvas.width,
        canvas.height
    );

    drawGrid();
}


function drawGrid() {
    const gridSize = 40;

    const gridColor = settings.theme === "light" ? "#999" : "#111";

    ctx.strokeStyle =
        gridColor;

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


// ============================================
// PLAYER
// ============================================

function drawPlayer() {
    const player = gameState.player;
    
    if (!player) {
        return;
    }

    const width = player.size * 2.8;
    const height = player.size * 2.8;

    ctx.shadowBlur = 12;
    ctx.shadowColor = "#00ff88";

    if (drawSprite("player", player.x, player.y, width, height, playerFacing)) {
        ctx.shadowBlur = 0;
        return;
    }

    ctx.shadowBlur = 0;

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

    ctx.strokeStyle =
        "#333";

    ctx.lineWidth = 2;

    ctx.stroke();

    ctx.closePath();
}


// ============================================
// ENERGY
// ============================================

function drawEnergy() {
    const energy = gameState.energy;
    
    if (!energy) {
        return;
    }

    ctx.shadowBlur = 20;
    ctx.shadowColor = "#00ff88";

    const width = energy.size * 2.8 * spritePulse();
    const height = energy.size * 2.8 * spritePulse();

    if (drawSprite("energy", energy.x, energy.y, width, height, 0)) {
        ctx.shadowBlur = 0;
        return;
    }

    ctx.shadowBlur = 0;

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

    ctx.fill();

    ctx.closePath();
}


// ============================================
// POWER-UPS
// ============================================

function drawPowerUps() {
    if (!gameState.powerups || !Array.isArray(gameState.powerups)) {
        return;
    }
    
    // `gameState.powerups` is the array that
    // Rust sent to the browser.
    //
    // `for...of` gives us one power-up at a time.
    for (
        const powerup
        of gameState.powerups
    ) {
        if (!powerup) continue;
        
        // Decide how the power-up should
        // look based on its type.
        //
        // Rust sends:
        //
        // "shield"
        // "speed"
        // "double_score"
        const style =
            getPowerUpStyle(
                powerup.kind
            );

        ctx.shadowBlur = 20;
        ctx.shadowColor = style.color;

        const width = powerup.size * 2.8 * spritePulse();
        const height = powerup.size * 2.8 * spritePulse();

        if (drawSprite(powerup.kind, powerup.x, powerup.y, width, height, 0)) {
            ctx.shadowBlur = 0;
            continue;
        }

        // Draw the power-up circle.
        ctx.beginPath();

        ctx.arc(
            powerup.x,
            powerup.y,
            powerup.size,
            0,
            Math.PI * 2
        );

        ctx.fillStyle =
            style.color;

        ctx.fill();

        ctx.closePath();

        // Turn off the shadow after
        // drawing the power-up.
        //
        // If we don't reset it, later objects
        // may accidentally get the same glow.
        ctx.shadowBlur = 0;

        // Draw a small letter inside the circle.
        drawPowerUpLabel(
            powerup,
            style
        );

    }
}


// Return the visual configuration for
// a particular power-up.
function getPowerUpStyle(
    kind
) {
    switch (kind) {
        case "shield":
            return {
                color: "#4488ff",
                label: "S",
            };

        case "speed":
            return {
                color: "#ffaa00",
                label: "⚡",
            };

        case "double_score":
            return {
                color: "#cc55ff",
                label: "2X",
            };

        // If the server somehow sends
        // an unknown power-up type,
        // use white as a fallback.
        default:
            return {
                color: "#ffffff",
                label: "?",
            };
    }
}


// Draw the label inside a power-up.
function drawPowerUpLabel(
    powerup,
    style
) {
    if (!powerup || !style) {
        return;
    }
    
    ctx.fillStyle =
        "#000000";

    ctx.font =
        "bold 10px Arial";

    ctx.textAlign =
        "center";

    ctx.textBaseline =
        "middle";

    ctx.fillText(
        style.label,
        powerup.x,
        powerup.y
    );
}


// ============================================
// ENEMIES
// ============================================

function drawEnemies() {
    if (!gameState.enemies || !Array.isArray(gameState.enemies)) {
        return;
    }
    
    for (
        const enemy
        of gameState.enemies
    ) {
        if (!enemy) continue;

        ctx.shadowBlur = 15;
        ctx.shadowColor = "#ff3355";

        const width = enemy.size * 2.8;
        const height = enemy.size * 2.8;

        if (drawSprite("enemy", enemy.x, enemy.y, width, height, 0)) {
            ctx.shadowBlur = 0;
            continue;
        }

        ctx.shadowBlur = 0;

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

        ctx.fill();

        ctx.closePath();
    }
}


// ============================================
// SIDEBAR CONTROLS
// ============================================

if (toggleSidebarButton) {
    toggleSidebarButton.addEventListener(
        "click",
        () => {
            if (sidebar) sidebar.classList.toggle("collapsed");
        }
    );
}

if (soundToggle) {
    soundToggle.addEventListener(
        "change",
        (e) => {
            settings.soundEnabled = e.target.checked;
            localStorage.setItem('soundEnabled', settings.soundEnabled);
        }
    );
}

if (musicToggle) {
    musicToggle.addEventListener(
        "change",
        (e) => {
            settings.musicEnabled = e.target.checked;
            localStorage.setItem('musicEnabled', settings.musicEnabled);
            
            if (settings.musicEnabled) {
                initAudio();
            } else {
                stopMusic();
            }
        }
    );
}

if (difficultySelect) {
    difficultySelect.addEventListener(
        "change",
        (e) => {
            settings.difficulty = e.target.value;
            localStorage.setItem('difficulty', settings.difficulty);
            
            // Send difficulty to server
            if (socket && socket.readyState === WebSocket.OPEN) {
                socket.send(JSON.stringify({
                    type: "difficulty",
                    difficulty: settings.difficulty
                }));
            }
        }
    );
}

if (themeSelect) {
    themeSelect.addEventListener(
        "change",
        (e) => {
            settings.theme = e.target.value;
            localStorage.setItem('theme', settings.theme);
            applyTheme(settings.theme);
        }
    );
}

if (resetStatsButton) {
    resetStatsButton.addEventListener(
        "click",
        () => {
            resetStatistics();
        }
    );
}

// ============================================
// PAUSE FUNCTIONALITY
// ============================================

function togglePause() {
    if (!gameState || gameState.status !== "running") {
        return;
    }
    
    gameStats.isPaused = !gameStats.isPaused;
    
    if (gameStats.isPaused) {
        gameStats.pauseStartTime = Date.now();
        if (pauseScreen) pauseScreen.classList.remove("hidden");
    } else {
        if (gameStats.pauseStartTime) {
            gameStats.totalPausedTime += Date.now() - gameStats.pauseStartTime;
            gameStats.pauseStartTime = null;
        }
        if (pauseScreen) pauseScreen.classList.add("hidden");
    }
    
    // Send pause state to server
    if (socket && socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify({
            type: "pause",
            paused: gameStats.isPaused
        }));
    }
}

if (resumeButton) {
    resumeButton.addEventListener(
        "click",
        () => {
            togglePause();
        }
    );
}

if (quitButton) {
    quitButton.addEventListener(
        "click",
        () => {
            togglePause();
            resetGame();
        }
    );
}

// ============================================
// THEME FUNCTIONALITY
// ============================================

function applyTheme(theme) {
    if (!document.body) {
        return;
    }
    
    document.body.className = '';
    if (theme !== 'dark') {
        document.body.classList.add(`theme-${theme}`);
    }
}

// ============================================
// AUDIO FUNCTIONALITY
// ============================================

function initAudio() {
    if (!audioContext) {
        try {
            audioContext = new (window.AudioContext || window.webkitAudioContext)();
        } catch (e) {
            console.warn("Audio context not supported:", e);
        }
    }
    
    // Resume audio context if it's suspended (browser requirement)
    if (audioContext && audioContext.state === 'suspended') {
        audioContext.resume();
    }
    
    startMusic();
}

function startMusic() {
    if (!settings.musicEnabled || !audioContext || musicNodes) {
        return;
    }
    
    try {
        const gain = audioContext.createGain();
        gain.gain.value = 0.015;
        
        const filter = audioContext.createBiquadFilter();
        filter.type = "lowpass";
        filter.frequency.value = 200;
        
        const osc1 = audioContext.createOscillator();
        osc1.type = "sine";
        osc1.frequency.value = 55;
        
        const osc2 = audioContext.createOscillator();
        osc2.type = "sine";
        osc2.frequency.value = 55.5;
        
        osc1.connect(filter);
        osc2.connect(filter);
        filter.connect(gain);
        gain.connect(audioContext.destination);
        
        osc1.start();
        osc2.start();
        
        musicNodes = { gain, filter, osc1, osc2 };
    } catch (e) {
        console.warn("Error starting music:", e);
    }
}

function stopMusic() {
    if (!musicNodes) {
        return;
    }
    
    try {
        musicNodes.osc1.stop();
        musicNodes.osc2.stop();
    } catch (e) {
        // Ignore stop errors
    }
    
    musicNodes = null;
}

function playSound(type) {
    if (!settings.soundEnabled) {
        return;
    }
    
    if (!audioContext) {
        initAudio();
        if (!audioContext) {
            return;
        }
    }
    
    try {
        const oscillator = audioContext.createOscillator();
        const gainNode = audioContext.createGain();
        
        oscillator.connect(gainNode);
        gainNode.connect(audioContext.destination);
        
        switch (type) {
            case 'collect':
                oscillator.frequency.setValueAtTime(800, audioContext.currentTime);
                oscillator.frequency.exponentialRampToValueAtTime(1200, audioContext.currentTime + 0.1);
                gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.1);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.1);
                break;
            case 'powerup':
                oscillator.frequency.setValueAtTime(400, audioContext.currentTime);
                oscillator.frequency.exponentialRampToValueAtTime(800, audioContext.currentTime + 0.2);
                gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.2);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.2);
                break;
            case 'gameover':
                oscillator.frequency.setValueAtTime(200, audioContext.currentTime);
                oscillator.frequency.exponentialRampToValueAtTime(50, audioContext.currentTime + 0.5);
                gainNode.gain.setValueAtTime(0.2, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.5);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.5);
                break;
        }
    } catch (e) {
        console.warn("Error playing sound:", e);
    }
}

// ============================================
// STATISTICS TRACKING
// ============================================

function updateStatistics() {
    if (highScoreElement) highScoreElement.textContent = gameStats.highScore;
    if (gamesPlayedElement) gamesPlayedElement.textContent = gameStats.gamesPlayed;
    if (totalTimeElement) totalTimeElement.textContent = formatTime(gameStats.totalTime);
}

function saveStatistics() {
    localStorage.setItem('highScore', gameStats.highScore);
    localStorage.setItem('gamesPlayed', gameStats.gamesPlayed);
    localStorage.setItem('totalTime', gameStats.totalTime);
}

function resetStatistics() {
    if (confirm('Are you sure you want to reset all statistics? This cannot be undone.')) {
        gameStats.highScore = 0;
        gameStats.gamesPlayed = 0;
        gameStats.totalTime = 0;
        saveStatistics();
        updateStatistics();
        console.log('Statistics reset successfully');
    }
}

function formatTime(seconds) {
    if (!seconds || seconds < 0) {
        return "0s";
    }
    
    if (seconds < 60) {
        return `${seconds}s`;
    } else if (seconds < 3600) {
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        return `${minutes}m ${remainingSeconds}s`;
    } else {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        return `${hours}h ${minutes}m`;
    }
}

// ============================================
// BUTTONS
// ============================================

if (startButton) {
    startButton.addEventListener(
        "click",
        () => {
            initAudio();
            startGame();
        }
    );
}

if (restartButton) {
    restartButton.addEventListener(
        "click",
        () => {
            resetGame();
        }
    );
}


// ============================================
// INITIALIZATION
// ============================================

function initialize() {
    // Load saved settings
    settings.soundEnabled = localStorage.getItem('soundEnabled') !== 'false';
    settings.musicEnabled = localStorage.getItem('musicEnabled') !== 'false';
    settings.difficulty = localStorage.getItem('difficulty') || 'normal';
    settings.theme = localStorage.getItem('theme') || 'dark';
    
    // Apply loaded settings to UI
    if (soundToggle) soundToggle.checked = settings.soundEnabled;
    if (musicToggle) musicToggle.checked = settings.musicEnabled;
    if (difficultySelect) difficultySelect.value = settings.difficulty;
    if (themeSelect) themeSelect.value = settings.theme;
    
    // Apply theme
    applyTheme(settings.theme);
    
    // Update statistics display
    updateStatistics();
    
    connectWebSocket();
}

initialize();