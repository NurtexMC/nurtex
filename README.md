# Nurtex

This is a library written in Rust that allows you to create Minecraft bots and manage them, including connection and packet processing. This library focuses on an asynchronous environment, maximum speed and optimization, and ease of coding.

Supported Minecraft version: `1.21.11` (or protocol version - `774`).

# Tasks

- [x] Connecting to servers
- [x] Login processing
- [x] Configuration processing
- [x] Processing of core game packets
- [ ] Processing of all game packets
- [x] Entity storage
- [x] Chunk storage
- [x] Ability to add handlers
- [x] Implementation of physics (partial and simplified)
- [ ] Interaction with inventory
- [ ] Interaction with entities
- [x] Bot control
- [x] Bot customization
- [x] Swarm architecture
- [x] Swarm control
- [x] Swarm customization
- [ ] Bypassing bot checks

# Usage

To use this library in your code, add a dependency to the Cargo.toml:

```
nurtex = "0.5.2"
```

# Examples

All current examples can be found here: [browse](https://github.com/NurtexMC/nurtex/tree/main/examples)

## Creating a bot

```rust
use std::io;

use nurtex::create_bot;
use nurtex::events::EventInvoker;

#[tokio::main]
async fn main() -> io::Result<()> {
  // Creating a bot
  let bot = crate_bot("NurtexBot");

  // Create an event invoker
  let mut event_invoker = EventInvoker::new();

  // Сreate a handler for the "spawn" event
  event_invoker.on_spawn(|terminal| async move {
    terminal.chat("Hello, world!").await;
  });

  bot
    .set_event_invoker(event_invoker) // Set event invoker
    .connect_to("localhost", 25565) // Connect bot to server
    .await
}
```