# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`simwork` is a discrete-event network simulator, built as a Rust library crate (edition 2024). It is early: `core::util` is complete and heavily tested, while the simulation engine in `core` is still type scaffolding — most structs have fields but no behaviour, so `cargo build` currently emits ~5 `field is never read` warnings. Those are expected until the engine is wired up.

## Commands

```sh
cargo test                                  # all tests (all live in-module)
cargo test bandwidth                        # every test in a module
cargo test bandwidth::tests::display_picks_the_largest_fitting_unit   # one test
cargo test -- --nocapture                   # keep test stdout
cargo build
cargo clippy --lib
cargo fmt
```

## Architecture

Two layers, in very different states of completion.

### `core::util` — the value types

`SimTime`, `Duration` and `Bandwidth`: everything the simulator measures goes through these, and they are the reference for style in this repo. Time is picoseconds everywhere, held in a `u128`; `Bandwidth` is a `u64` of bits/sec and converts back to time via `transmission_time` / `bits_in`. See `src/core/util/CLAUDE.md` for the conventions these types share — read it before touching or extending them.

### `core` — the simulation engine (scaffolding)

`Runner` is the intended driver: it holds `now: SimTime` plus `HashMap`s of `Node`s and `Channel`s. A `Channel` is a directed head→tail link between two nodes carrying a propagation `Duration`. `Event` is the unit of simulation work (`AppStart`, `AppEnd`, `SendPacket`, `ReceivePacket`), pulled from an `EventQueue` built on `BTreeSet<Reverse<Event>>` — so `Event` will need an `Ord` that sorts by scheduled `SimTime` before the queue can be used at all.

`Packet` is recursively nested (`data: Box<Packet>`) to model encapsulation, with `Header` wrapping `etherparse` header types. `smoltcp` is a declared dependency but not yet used. `address.rs` is empty, and `ChannelId::new` is a stub returning `0` with a TODO for real id generation.

## Style

- Do not write comments for what the type system already expresses. Doc comments earn their place by explaining truncation, panics, `None` cases, units, or a non-obvious choice of width — not by restating the signature.
- Tests live in the same file under `#[cfg(test)] mod tests`, and are named as a sentence about the behaviour (`each_unit_is_a_thousandfold_of_the_next`, `transmission_time_reports_a_dead_link`). Prefer asserting a property or an edge over re-checking arithmetic that the constructors already guarantee.
