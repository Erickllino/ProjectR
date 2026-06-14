# Mini MMO RPG — Project Architecture
 
> A reference document for building a multiplayer RPG in **Rust + Bevy**, with a focus on learning transferable skills (networking, databases, server architecture).
 
---
 
## 1. What we're building
 
A "Mini MMO" — same core concepts as a real MMO, but scoped so it's actually finishable as a learning project.
 
```
Players log in → Pick/create a character → Enter a shared world
→ See other players moving in real time → Fight enemies → Level up
→ Log out → Come back → Character is still there
```
 
This single loop teaches everything important: authentication, databases, real-time multiplayer, server-authoritative game logic, and persistence.
 
---
 
## 2. The tech stack
 
| Layer | Technology | Why |
|---|---|---|
| Game client | **Bevy** (Rust game engine) | Native binary → ships on Steam; ECS architecture |
| Server | **Rust + tokio** (async runtime) | Same language as client; fast; safe |
| Networking | **lightyear** (or similar) | Real-time client ↔ server messaging |
| Auth API | **axum** | HTTP endpoints for login/register |
| Database | **PostgreSQL** + `sqlx` | Industry standard; persists all game state |
| Password security | **argon2** | Secure password hashing |
 
---
 
## 3. The three layers
 
The project has three conceptual layers. Each has one job.
 
### Client (Bevy game)
What the player sees and controls. Renders sprites, the map, and UI; reads keyboard/mouse input; sends inputs to the server; displays whatever the server says is true. **Ships to players via Steam.**
 
### Server (Rust)
The *authority* — it decides what is actually real. Runs the game simulation, validates everything clients send, talks to the database. **Never ships to players; runs on a machine you rent.**
 
### Database (PostgreSQL)
Long-term memory. Stores accounts, characters, stats, inventory. Survives server restarts so players' progress persists between sessions.
 
---
 
## 4. The two key mental models
 
These two ideas are what make the whole architecture make sense.
 
### A) Only the client ships
 
The development workspace contains `client/`, `server/`, and `shared/` all together — that's just where you *write* the code. When you build it, you get **two separate programs**:
 
```
Dev workspace (client + server + shared)
        │
        ├──► Client binary ──► Steam ──► Player's computer
        │
        └──► Server binary ──► Your rented server (always running)
```
 
The player downloads **only** the client. Your server code stays with you. The `shared` crate gets compiled into both, but the player only ever receives the client half.
 
> A **VPS** (Virtual Private Server) is a computer in a data center you rent (~$5–20/month) to keep the server running 24/7.
 
### B) The server is the authority
 
**Game logic lives on the server.** The client does NOT decide anything important.
 
```
Client: "I pressed the right arrow key"      → sends to server
Server: checks for walls, stuns, etc.
Server: "OK, you're now at x=42"             → sends back to client
Client: draws the player at x=42
```
 
The client says what it *wants* to do; the server decides what *actually* happens.
 
**Why?** Anti-cheat. If the client decided things, a hacker could edit it to say "I have 9999 HP" or "I hit every enemy." Because the server is the authority, it ignores anything it didn't validate itself.
 
| Lives on the **server** | Lives on the **client** |
|---|---|
| Combat math (did you hit?) | Drawing sprites |
| Movement validation | Playing sounds |
| Loot, XP, leveling | Reading keyboard input |
| Anything cheatable | Smooth animations |
 
> *Nuance for later:* clients often "predict" their own movement so it feels instant instead of waiting for the round-trip — but the server always has final say. Not needed early on.
 
---
 
## 5. Project structure
 
```
my-mmo/
├── Cargo.toml          ← workspace root
│
├── client/             ← Bevy game (ships to players)
│   └── src/
│       ├── main.rs     ← sets up the Bevy app, plugins, window
│       ├── player.rs   ← player entity, input, sprite movement
│       ├── world.rs    ← map, tiles, camera
│       ├── network.rs  ← connects to server, sends/receives messages
│       ├── ui.rs       ← login screen, health bar, chat, inventory
│       └── states.rs   ← which "screen": MainMenu / LoggingIn / Playing
│
├── server/             ← Rust server (never ships)
│   └── src/
│       ├── main.rs     ← starts tokio, DB pool, routes, game loop
│       ├── auth.rs     ← login/register LOGIC + password hashing
│       ├── db.rs       ← all raw database queries
│       ├── game_loop.rs← the server "tick": physics, combat, AI
│       └── network.rs  ← receives inputs, broadcasts world state
│
└── shared/             ← used by BOTH client and server
    └── src/
        ├── lib.rs      ← crate root, exports the below
        ├── messages.rs ← network message types
        ├── components.rs← shared game data (Position, Health…)
        └── constants.rs← tick rate, player speed, map size
```
 
### Why `shared` exists
 
Both sides import the same types, so the **compiler** verifies they agree. If the client sends a `Move` message but the server expects different fields, the code won't even compile — the bug is caught before you ship, instead of at runtime with players already stuck.
 
**Rule of thumb for where code goes:**
- Does the server need it too? → `shared`
- Only about *drawing*? → `client`
- Only about *deciding what's real*? → `server`
---
 
## 6. Where data is stored
 
All persistent data lives in **PostgreSQL**. Starter tables:
 
```sql
CREATE TABLE players (
    id            SERIAL PRIMARY KEY,
    username      VARCHAR(32) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,   -- a HASH, never the real password
    created_at    TIMESTAMP DEFAULT NOW()
);
 
CREATE TABLE characters (
    id         SERIAL PRIMARY KEY,
    player_id  INTEGER REFERENCES players(id),
    name       VARCHAR(32),
    level      INTEGER DEFAULT 1,
    pos_x      REAL DEFAULT 0,
    pos_y      REAL DEFAULT 0
);
```
 
Other tables you'll add later: `inventory` (item, quantity), `world_state` (events, spawns).
 
> **Security rule:** never store the real password. Store a **hash** (via `argon2`). When someone logs in, hash what they typed and compare hashes. Even you, the developer, should never be able to see a player's password.
 
---
 
## 7. Example flow — registering an account
 
```
1. Client      → player types username + password into the UI form
2. Client      → (optional cosmetic check: fields not empty, passwords match)
3. Client      → sends credentials to server over an ENCRYPTED connection
4. auth.rs     → validates input, checks username is available
5. auth.rs     → hashes the password with argon2
6. db.rs       → INSERT INTO players (...)
7. PostgreSQL  → row saved to disk
8. Server      → responds "account created" (or an error)
```
 
Notice `auth.rs` (the *logic*) and `db.rs` (the *queries*) are separate. `auth.rs` decides the rules; `db.rs` just runs SQL and doesn't know why. Real codebases keep this split everywhere.
 
> The connection must be **encrypted** (TLS — the padlock tech in browsers) so the password can't be read in transit. Not needed on day one, but it's why sending the password is safe in practice.
 
---
 
## 8. Learning roadmap
 
You don't know much Rust yet — that's fine. Don't start with Bevy; it'll feel like magic black boxes without the Rust underneath.
 
### Phase 0 — Just enough Rust (2–4 weeks)
- **Week 1:** variables, functions, structs, enums, `match`, `Option`/`Result`
- **Week 2:** ownership & borrowing (Rust's famously weird part — persevere)
- **Week 3:** traits, `impl` blocks, basic iterators
**Resources:** The Rust Book (official, free), Rustlings (exercises — do these alongside the book), No Boilerplate (YouTube).
 
### Phase 1 — Bevy basics (2–3 weeks)
First project: **a character that moves around a 2D map with arrow keys.** Tiny, but teaches the Bevy app setup, ECS, input, and transforms — everything the MMO needs. Use the **Bevy Cheatbook**.
 
### Phase 2+ — The actual MMO, in phases
Each phase is a complete, working game on its own:
1. **Foundation** — accounts (login/register) + database storing characters
2. **The World** — 2D map, your character moving
3. **Multiplayer** — other players appear, movement synced via server
4. **RPG Systems** — stats, leveling, basic combat, enemies
5. **Polish** — inventory, quests, chat, whatever excites you
---
 
## 9. Quick glossary
 
| Term | Meaning |
|---|---|
| **ECS** | Entity Component System — Bevy's architecture: entities (objects) have components (data) that systems (logic) act on |
| **Authoritative server** | The server, not the client, decides what's real |
| **VPS** | A rented always-on computer in a data center, for hosting the server |
| **Hash** | A one-way scramble of a password; can't be reversed |
| **TLS** | Encryption for data in transit (the browser padlock) |
| **Crate** | A Rust package/library |
| **tokio** | Rust's async runtime, lets the server handle many players at once |
 
---
 
*Bon courage — you've got a solid plan. Build small, ship at every phase.* 🎮
 
