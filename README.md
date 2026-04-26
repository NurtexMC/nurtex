# Nurtex

A collection of lightweight Rust libraries for creating Minecraft bots. Async, optimized, ease of coding.


# Focusing

All crates focus on:

- **Speed:** All crates do not contain heavy, long-term operations, even if they do, they are optimized.
- **Lightweight:** All crates do not carry any extra dependencies or code.
- **Asynchrony:** Almost all crates rely on an asynchronous code environment.
- **Simplicity:** We try to make the logic of crates understandable for everyone, or at least organize simple use of crates.


# Tasks and Goals

- [x] Bot architecture
- [x] Swarm architecture
- [x] Connecting to servers
- [x] Encryption
- [x] Login processing
- [x] Configuration processing
- [ ] Play processing (a partial implementation already exists)
- [x] Auxiliary functionality (plugins, speedometer, just functions and methods)
- [ ] Implementation of physics
- [ ] Interaction with inventory
- [ ] Interaction with entities
- [x] Flexible settings (relative to the current position)
- [ ] NBT passring
- [ ] Text component parsing (it's there now, but it doesn't work as it should)
- [ ] Basic bypass of client validity checks (planned to be implemented soon)
- [ ] Bypass primitive bot checks
- [ ] Bypass complex bot checks (complete imitation of a real player)


# Crate map

- [nurtex](https://github.com/NurtexMC/nurtex/tree/main/crates/nurtex): A crate for high-level work with the bot / swarm API.
- [nurtex-codec](https://github.com/NurtexMC/nurtex/tree/main/crates/nurtex-codec): A crate for serializing Minecraft types into Rust types.
- [nurtex-derive](https://github.com/NurtexMC/nurtex/tree/main/crates/nurtex-derive): A crate for convenient parsing of network packets.
- [nurtex-encrypt](https://github.com/NurtexMC/nurtex/tree/main/crates/nurtex-encrypt): A crate containing the Minecraft TCP-connection encryption.
- [nurtex-protocol](https://github.com/NurtexMC/nurtex/tree/main/crates/nurtex-protocol): A crate for creating Minecraft TCP-connections and working with packets.