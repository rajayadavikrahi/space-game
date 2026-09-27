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

const lampHudElement =
    document.getElementById(
        "lampHud"
    );

const lampStatusElement =
    document.getElementById(
        "lampStatus"
    );

const lampBatteryElement =
    document.getElementById(
        "lampBattery"
    );

const WS_URL =
    window.VOIDRUNNER_BACKEND_URL ||
    "ws://127.0.0.1:3000/ws";

// ============================================
// LAMP CONSTANTS
// ============================================
//
// These mirror `rust/src/game/light.rs`. The server
// owns the real values, these are only used for
// the frames before the first state message and
// for drawing the dark.

const MAX_BATTERY = 100;

const MAX_LAMP_RADIUS = 270;

const MIN_LAMP_RADIUS = 90;

// How far you can see with the lamp switched off.
const AMBIENT_RADIUS = 70;

// Angle of the beam the lamp throws in the
// direction the player is facing.
const BEAM_ANGLE = Math.PI / 3.4;

// How black the unlit part of the arena is.
const DARKNESS_ALPHA = 0.97;

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

// Local copy of the lamp switch. The server is the
// real source of truth, this only exists so the
// screen reacts on the same frame the key is
// pressed instead of on the next state message.
let lampOn = true;

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

            // F (or L) works the lamp switch.
            case "f":
            case "l":
                toggleLamp();
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
// LAMP
// ============================================
//
// The grid is down and the lamp is the only light
// in the arena, so the lamp switch is a game
// mechanic and not a cosmetic toggle:
//
// * the lamp burns battery, the server decides
//   when it dies,
// * energy orbs can only be collected while the
//   lamp is burning,
// * and enemies inside the beam move faster.
//
// The server owns the state, this side only reports
// the switch and draws what comes back.

function lampData() {
    if (gameState && gameState.light) {
        return gameState.light;
    }

    return null;
}

function currentBattery() {
    const light = lampData();

    return light ? light.battery : MAX_BATTERY;
}

// Is the lamp producing light right now?
function isLampLit() {
    const light = lampData();

    if (light) {
        return light.lit;
    }

    return lampOn;
}

// How far the light reaches, in pixels. This is
// the server value, so the beam visibly shrinks as
// the battery runs down.
function lampRadius() {
    const light = lampData();

    if (!light) {
        return MAX_LAMP_RADIUS;
    }

    if (!light.lit) {
        return 0;
    }

    return light.radius;
}

function sendLampState(on) {
    if (
        !socket ||
        socket.readyState !==
            WebSocket.OPEN
    ) {
        return;
    }

    lampOn = on;

    socket.send(
        JSON.stringify({
            type: "light",

            on: on,
        })
    );

    playSound(on ? "lampOn" : "lampOff");
}

function toggleLamp() {
    if (
        !gameState ||
        gameState.status !== "running"
    ) {
        return;
    }

    // A dead lamp cannot be switched back on. The
    // battery has to recharge first, which only
    // happens while the lamp is off.
    if (!lampOn && currentBattery() <= 0) {
        playSound("lampDead");
        return;
    }

    sendLampState(!lampOn);
}

function updateLampHud() {
    if (!lampHudElement) {
        return;
    }

    const battery =
        currentBattery();

    const lit =
        isLampLit();

    const percent =
        Math.max(
            0,
            Math.min(
                1,
                battery / MAX_BATTERY
            )
        );

    if (lampStatusElement) {
        // DEAD means the battery is flat and the
        // lamp is recharging in the dark.
        lampStatusElement.textContent =
            lit ? "ON" : battery > 0 ? "OFF" : "DEAD";
    }

    if (lampBatteryElement) {
        lampBatteryElement.style.width =
            (percent * 100).toFixed(1) + "%";
    }

    lampHudElement.classList.toggle("lamp-on", lit);
    lampHudElement.classList.toggle("lamp-off", !lit);
    lampHudElement.classList.toggle("lamp-low", lit && percent < 0.25);
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

    // The server decides whether the lamp is lit,
    // so follow it instead of trusting the key we
    // pressed a moment ago.
    if (gameState.light) {
        lampOn = gameState.light.on;
    }

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

    // Update lamp status
    updateLampHud();
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

loadSprite("lantern", `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <defs>
    <radialGradient id="flame" cx="0.5" cy="0.45" r="0.5">
      <stop offset="0" stop-color="#ffffff"/>
      <stop offset="0.5" stop-color="#ffe066"/>
      <stop offset="1" stop-color="#ff9d00" stop-opacity="0"/>
    </radialGradient>
  </defs>
  <circle cx="32" cy="32" r="28" fill="#2a2205" stroke="#ffe066" stroke-width="3"/>
  <path d="M24 18 h16" stroke="#c9a227" stroke-width="3" stroke-linecap="round"/>
  <path d="M20 20 L14 44 a3 3 0 0 0 3 4 h30 a3 3 0 0 0 3 -4 L44 20 Z" fill="#3a3208" stroke="#c9a227" stroke-width="2" stroke-linejoin="round"/>
  <circle cx="32" cy="32" r="14" fill="url(#flame)"/>
  <path d="M32 24 c4 5 6 8 6 11 a6 6 0 0 1 -12 0 c0 -3 2 -6 6 -11 Z" fill="#fff3c4"/>
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

    // The blackout itself. Everything the lamp
    // does not reach is painted over in black.
    drawDarkness();

    // Warm spill from the lamp, drawn on top so
    // the beam feels like light and not like a
    // hole in the screen.
    drawLampGlow();
}


// ============================================
// LIGHT AND DARKNESS
// ============================================

// How visible is something at (x, y)?
//
// This is what makes the light matter in the light
// theme, where the arena background stays bright:
// whatever the lamp does not reach is only a hint.
function visibilityAt(x, y) {
    const player =
        gameState.player;

    if (!player) {
        return 1;
    }

    const distance =
        Math.hypot(
            x - player.x,
            y - player.y
        );

    const radius =
        lampRadius();

    if (radius <= 0) {
        // Lamp off. Only a small halo of ambient
        // light around the player.
        if (distance <= AMBIENT_RADIUS) {
            return 0.45;
        }

        return 0.1;
    }

    // Fully lit core, then a falloff, then
    // barely anything.
    const core = radius * 0.7;

    let level =
        distance <= core
            ? 1
            : distance <= radius
              ? 1 - 0.4 * ((distance - core) / (radius - core))
              : 0.25;

    // The beam reaches past the halo, so anything
    // inside the cone counts as lit even when it
    // sits outside the radius.
    //
    // The cone is worked out from the direction the
    // ship points, which is the local -y axis of the
    // sprite, turned by the facing angle.
    const length = radius * 1.9;

    if (distance <= length && distance > 0) {
        const along =
            (x - player.x) * Math.sin(playerFacing) -
            (y - player.y) * Math.cos(playerFacing);

        const across =
            (x - player.x) * Math.cos(playerFacing) +
            (y - player.y) * Math.sin(playerFacing);

        if (
            along > 0 &&
            Math.abs(across) <= along * Math.tan(BEAM_ANGLE / 2)
        ) {
            level =
                Math.max(
                    level,
                    1 - 0.3 * (distance / length)
                );
        }
    }

    return level;
}

// The mask is built on a canvas of its own and then
// laid over the arena.
//
// It cannot be painted straight onto the arena:
// filling with black would destroy the picture
// underneath, and erasing that black again with
// `destination-out` only removes it, it does not
// bring the arena back.
let darknessCanvas = null;
let darknessCtx = null;

function darknessMask() {
    if (
        !darknessCanvas ||
        darknessCanvas.width !== canvas.width ||
        darknessCanvas.height !== canvas.height
    ) {
        darknessCanvas =
            document.createElement("canvas");

        darknessCanvas.width = canvas.width;
        darknessCanvas.height = canvas.height;

        darknessCtx =
            darknessCanvas.getContext("2d");
    }

    return darknessCtx;
}

// Lay the blackout over the arena, with the lit area
// and the beam cut out of it.
//
// The light theme keeps its bright background, so
// there `visibilityAt` is enough and the mask is
// skipped.
function drawDarkness() {
    const player =
        gameState.player;

    if (!player || !ctx) {
        return;
    }

    if (settings.theme === "light") {
        return;
    }

    const radius =
        lampRadius();

    const lit =
        radius > 0;

    // With the lamp off we still get a small
    // ambient halo, otherwise the player would be
    // completely blind.
    const halo =
        lit
            ? radius
            : AMBIENT_RADIUS;

    const mask =
        darknessMask();

    mask.setTransform(1, 0, 0, 1, 0, 0);

    mask.globalCompositeOperation = "source-over";

    mask.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
    );

    mask.fillStyle =
        "rgba(0, 0, 0, " + DARKNESS_ALPHA + ")";

    mask.fillRect(
        0,
        0,
        canvas.width,
        canvas.height
    );

    // Cutting the lit areas out of the mask.
    mask.globalCompositeOperation = "destination-out";

    punchHalo(
        mask,
        player.x,
        player.y,
        halo,
        lit ? 0.97 : 0.6
    );

    if (lit) {
        punchBeam(
            mask,
            player.x,
            player.y,
            playerFacing,
            BEAM_ANGLE,
            radius * 1.9
        );
    }

    ctx.drawImage(
        darknessCanvas,
        0,
        0
    );
}

// Erase a soft circle out of the mask.
function punchHalo(target, x, y, radius, strength) {
    if (radius <= 0) {
        return;
    }

    const gradient =
        target.createRadialGradient(
            x,
            y,
            0,
            x,
            y,
            radius
        );

    gradient.addColorStop(0, "rgba(0, 0, 0, " + strength + ")");
    gradient.addColorStop(0.55, "rgba(0, 0, 0, " + strength * 0.55 + ")");
    gradient.addColorStop(1, "rgba(0, 0, 0, 0)");

    target.fillStyle = gradient;

    target.beginPath();

    target.arc(
        x,
        y,
        radius,
        0,
        Math.PI * 2
    );

    target.fill();
}

// Erase a soft cone of darkness in front of the
// player, so the lamp is directional.
//
// The cone is built along the local -y axis,
// which is the way the player sprite points, so
// the beam always leaves the nose of the ship.
function punchBeam(target, x, y, angle, spread, length) {
    target.save();

    target.translate(x, y);
    target.rotate(angle);

    const gradient =
        target.createLinearGradient(
            0,
            0,
            0,
            -length
        );

    gradient.addColorStop(0, "rgba(0, 0, 0, 0.95)");
    gradient.addColorStop(0.45, "rgba(0, 0, 0, 0.6)");
    gradient.addColorStop(1, "rgba(0, 0, 0, 0)");

    target.fillStyle = gradient;

    target.beginPath();

    target.moveTo(0, 0);

    target.arc(
        0,
        0,
        length,
        -Math.PI / 2 - spread / 2,
        -Math.PI / 2 + spread / 2
    );

    target.closePath();

    target.fill();

    target.restore();
}

// Warm light spilling out of the lamp.
//
// A dying battery flickers, which is the warning
// that the dark is about to take over.
function drawLampGlow() {
    const player =
        gameState.player;

    const radius =
        lampRadius();

    if (!player || radius <= 0 || !ctx) {
        return;
    }

    const now =
        typeof performance !== "undefined"
            ? performance.now()
            : Date.now();

    const battery =
        currentBattery() / MAX_BATTERY;

    // A healthy lamp is steady, a dying one
    // flickers.
    const flicker =
        battery < 0.3
            ? 0.75 +
              0.25 *
                  Math.abs(
                      Math.sin(now / 60)
                  )
            : 1;

    const strength =
        (0.05 + 0.13 * battery) * flicker;

    ctx.save();

    ctx.globalCompositeOperation = "lighter";

    const gradient =
        ctx.createRadialGradient(
            player.x,
            player.y,
            0,
            player.x,
            player.y,
            radius
        );

    gradient.addColorStop(0, "rgba(255, 236, 180, " + strength + ")");
    gradient.addColorStop(0.6, "rgba(255, 214, 130, " + strength * 0.35 + ")");
    gradient.addColorStop(1, "rgba(255, 200, 100, 0)");

    ctx.fillStyle = gradient;

    ctx.beginPath();

    ctx.arc(
        player.x,
        player.y,
        radius,
        0,
        Math.PI * 2
    );

    ctx.fill();

    // Beam highlight, so the direction the player
    // is looking is obvious.
    ctx.globalAlpha = 0.5 * flicker;

    ctx.save();

    ctx.translate(player.x, player.y);
    ctx.rotate(playerFacing);

    const beam =
        ctx.createLinearGradient(
            0,
            0,
            0,
            -radius * 1.6
        );

    beam.addColorStop(0, "rgba(255, 245, 210, " + strength + ")");
    beam.addColorStop(1, "rgba(255, 245, 210, 0)");

    ctx.fillStyle = beam;

    ctx.beginPath();

    ctx.moveTo(0, 0);

    ctx.arc(
        0,
        0,
        radius * 1.6,
        -Math.PI / 2 - BEAM_ANGLE / 2,
        -Math.PI / 2 + BEAM_ANGLE / 2
    );

    ctx.closePath();

    ctx.fill();

    ctx.restore();

    ctx.restore();
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

    // Orbs outside the lamp are only a hint.
    ctx.globalAlpha =
        visibilityAt(energy.x, energy.y);

    ctx.shadowBlur = 20;
    ctx.shadowColor = "#00ff88";

    const width = energy.size * 2.8 * spritePulse();
    const height = energy.size * 2.8 * spritePulse();

    if (drawSprite("energy", energy.x, energy.y, width, height, 0)) {
        ctx.shadowBlur = 0;
        ctx.globalAlpha = 1;
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

    ctx.globalAlpha = 1;
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
        
        // Power-ups the lamp does not reach are
        // hard to make out.
        ctx.globalAlpha =
            visibilityAt(powerup.x, powerup.y);
        
        // Decide how the power-up should
        // look based on its type.
        //
        // Rust sends:
        //
        // "shield"
        // "speed"
        // "double_score"
        // "lantern"
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
            ctx.globalAlpha = 1;
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

        ctx.globalAlpha = 1;

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

            // The lantern. This one is not a
            // stat boost, it refills the lamp.
            case "lantern":
                return {
                    color: "#ffe066",
                    label: "L",
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

        // This is the blackout rule in one line: an
        // enemy the lamp does not reach is barely
        // there, so switching the light off is how
        // you buy yourself a moment.
        ctx.globalAlpha =
            visibilityAt(enemy.x, enemy.y);

        ctx.shadowBlur = 15;
        ctx.shadowColor = "#ff3355";

        const width = enemy.size * 2.8;
        const height = enemy.size * 2.8;

        if (drawSprite("enemy", enemy.x, enemy.y, width, height, 0)) {
            ctx.shadowBlur = 0;
            ctx.globalAlpha = 1;
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

        ctx.globalAlpha = 1;
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
            case 'lampOn':
                oscillator.type = "square";
                oscillator.frequency.setValueAtTime(320, audioContext.currentTime);
                oscillator.frequency.exponentialRampToValueAtTime(640, audioContext.currentTime + 0.06);
                gainNode.gain.setValueAtTime(0.06, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.08);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.08);
                break;
            case 'lampOff':
                oscillator.type = "square";
                oscillator.frequency.setValueAtTime(640, audioContext.currentTime);
                oscillator.frequency.exponentialRampToValueAtTime(320, audioContext.currentTime + 0.06);
                gainNode.gain.setValueAtTime(0.06, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.08);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.08);
                break;
            case 'lampDead':
                // The click of a lamp that refuses to
                // come on while the battery is flat.
                oscillator.type = "square";
                oscillator.frequency.setValueAtTime(180, audioContext.currentTime);
                gainNode.gain.setValueAtTime(0.05, audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.05);
                oscillator.start(audioContext.currentTime);
                oscillator.stop(audioContext.currentTime + 0.05);
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