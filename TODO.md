# Mini MMO — TODO List

A phased, ordered checklist to get from "menu prototype" to "working multiplayer game."
Each phase is a complete, runnable thing on its own. **Don't skip ahead** — every phase
depends on the one before it. See `PLAN.md` for the architecture and the *why*.

Legend: `[ ]` todo · `[~]` partially done · `[x]` done

---

## Current state (snapshot)

- [x] Cargo workspace (`client` / `server` / `shared`) compiles
- [x] Client menu state machine (Splash → Menu → Game)
- [x] Settings menu (display quality + volume)
- [x] Login screen **UI** (buttons render)
- [~] Auth flow — *fake*: the Login button just switches state, validates nothing
- [~] "Game" — it's the Bevy **Breakout** example acting as a placeholder
- [ ] Server — empty `println!`, no logic, no deps
- [ ] `shared` crate — still the stub `add()` function, no real types
- [ ] Networking — does not exist yet
- [ ] Database — only `db/users.json` with **plaintext passwords** (prototype only)

---

## Phase 0 — Rust fundamentals (ongoing, do alongside everything)

- [ ] Work through The Rust Book chs. 1–10 (structs, enums, `match`, `Option`/`Result`, ownership, traits)
- [ ] Do Rustlings exercises in parallel
- [ ] Be comfortable reading a compiler error and fixing it without panicking
> You already have real Bevy code running, so you're past "zero." Keep the book open as a reference.

---

## Phase 1 — Make the client a real single-player world (no server yet)

Goal: replace Breakout with a top-down character you can walk around. **No networking.**

- [ ] Add a `world` module: spawn a camera that follows the player, a simple tiled/colored background
- [ ] Add a `player` module: spawn a sprite entity with a `Position`
- [ ] Read WASD / arrow keys → move the player sprite each frame
- [ ] Clamp movement to map bounds (walls)
- [ ] Make the camera follow the player
- [ ] Delete or archive the Breakout code in `game/game.rs` once the new world works
- [ ] (Optional) Load a real sprite from `assets/` instead of a colored rectangle

**Done when:** you launch the client, click past the menu, and walk a character around a map.

---

## Phase 2 — Real local accounts (still no server)

Goal: login/register actually work, reading/writing a local file. This is a stepping stone
so the *flow* is real before you move the logic to a server.

- [ ] Add text-input fields to the login screen (capture typed username + password)
- [ ] Build the register screen (`register_setup` is currently an empty stub)
- [ ] On Register: append a new user; reject duplicate usernames
- [ ] On Login: check the username/password actually match before entering the game
- [ ] Show an on-screen error message on bad login / taken username
- [ ] **Hash passwords** with `argon2` instead of storing plaintext (replace the plaintext `db/users.json`)
- [ ] Persist a logged-in character's position back to the file on quit, load it on login

**Done when:** you register, quit, relaunch, log in, and your character is where you left it.

> This whole phase will later move to the server — that's expected and fine. The point is to
> learn the logic somewhere simple first.

---

## Phase 3 — Stand up the server + database

Goal: a real backend the client can talk to over HTTP.

- [ ] Add server deps: `tokio`, `axum`, `sqlx` (postgres), `argon2`, `serde`
- [ ] Install PostgreSQL locally; create the `players` and `characters` tables from `PLAN.md` §6
- [ ] `db.rs`: connection pool + raw queries (insert player, fetch by username, save character pos)
- [ ] `auth.rs` (server): register/login *logic* — validate input, hash password, call `db.rs`
- [ ] `main.rs` (server): start tokio, build axum router, expose `POST /register` and `POST /login`
- [ ] Test the endpoints with `curl` before touching the client
- [ ] Point the client's login/register screens at the server instead of the local file
- [ ] Use an env var / config for the DB connection string (never hard-code credentials)

**Done when:** the client registers + logs in by talking to your server, and data lands in Postgres.

---

## Phase 4 — Define the shared protocol

Goal: the `shared` crate holds the types both sides agree on. Do this *before* real-time multiplayer.

- [ ] Remove the stub `add()` from `shared/src/lib.rs`
- [ ] `shared/components.rs`: `Position`, `Health`, `PlayerId`, etc. (derive `Serialize`/`Deserialize`)
- [ ] `shared/messages.rs`: client→server (`Move`, `Attack`) and server→client (`WorldSnapshot`, `PlayerJoined`)
- [ ] `shared/constants.rs`: tick rate, player speed, map size
- [ ] Add `shared` as a dependency of both `client` and `server`

**Done when:** both crates import the same message types and still compile.

---

## Phase 5 — Real-time multiplayer (the actual MMO moment)

Goal: two clients see each other move, with the **server as the authority**.

- [ ] Choose a networking approach (`lightyear`, or start simpler with raw `tokio` TCP/UDP + `serde`)
- [ ] Server: accept client connections, track connected players, run a fixed-rate game loop (tick)
- [ ] Client sends *inputs* ("I want to move right"), not positions
- [ ] Server validates movement (walls, speed) and updates authoritative positions
- [ ] Server broadcasts a world snapshot each tick; client draws other players from it
- [ ] Handle join/leave: spawn/despawn other players' sprites
- [ ] Persist character position to Postgres periodically + on disconnect

**Done when:** you run two client windows and watch one move in the other in real time.

---

## Phase 6 — RPG systems

- [ ] Stats (HP, attack, defense) in `shared` components + DB
- [ ] XP and leveling (server-authoritative)
- [ ] Basic enemies with simple server-side AI (wander / chase)
- [ ] Combat: client requests attack → server does the math → broadcasts result
- [ ] Health bars + damage feedback on the client

**Done when:** you can fight an enemy, take/deal damage, gain XP, and level up — all decided by the server.

---

## Phase 7 — Polish (pick what excites you)

- [ ] Inventory + items (`inventory` table)
- [ ] Chat
- [ ] Quests
- [ ] TLS on the connection (encrypt login traffic) — see `PLAN.md` §7
- [ ] Deploy the server to a VPS so others can connect
- [ ] Sound/animation polish, better art

---

## Cross-cutting (good habits, start early)

- [ ] Add `db/users.json` to `.gitignore` (don't commit credentials, even test ones)
- [ ] Commit at the end of each working sub-step with a clear message
- [ ] Write a couple of tests for server auth logic (hashing, duplicate-username rejection)
- [ ] Keep a short dev log of what broke and how you fixed it (great for learning)
