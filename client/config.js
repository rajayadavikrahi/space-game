// WebSocket backend location.
//
// Local development: the Rust server runs on your
// machine at 127.0.0.1:3000 (keep this — used when
// the page is served from localhost).
//
// Production: the Rust backend is deployed on Render.
// Vercel and other remote hosts use this wss:// URL.
window.VOIDRUNNER_BACKEND_URL =
  location.hostname === "127.0.0.1" || location.hostname === "localhost"
    ? "ws://127.0.0.1:3000/ws"
    : "wss://api-space-game.onrender.com/ws";
