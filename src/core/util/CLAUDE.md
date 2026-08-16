# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Scope

`core::util` holds the value types every measurement in the simulator passes through. It is the one complete, heavily tested part of the crate, and is the reference for style everywhere else. Five modules, no submodules: `time` (`SimTime`), `duration` (`Duration`), `bandwidth` (`Bandwidth`), `size` (`Size`, `SizeOf`) and `id` (`IdGenerator`).

The first four are the value types and share every convention below. `id` is the odd one out — a mutable counter, not a measurement — and is described separately at the end.

## Commands

```sh
cargo test bandwidth                        # every test in a module
cargo test bandwidth::tests::display_picks_the_largest_fitting_unit   # one test
cargo test -- --nocapture                   # keep test stdout
cargo clippy --lib
cargo fmt
```

Tests live in-module under `#[cfg(test)] mod tests`, so the module path is the test filter.

## The types

**Time is picoseconds, everywhere.** `SimTime` is a point in time (ps since `SimTime::EPOCH`), `Duration` is a span; both wrap a `u128`. `SimTime - SimTime = Duration`, and `SimTime ± Duration = SimTime`. The width is deliberate: a `u64` of picoseconds tops out at ~5124 hours, so `u128` is what makes the wide constructors (`Duration::hour`, `SimTime::from_hour`) meaningful.

**`Bandwidth`** wraps a `u64` of bits/sec and is the bridge back to time: `transmission_time(bits)` gives the serialisation delay, `bits_in(duration)` / `bytes_in(duration)` give the volume carried. `transmission_time` and `bits_in` are inverses, and there is a test asserting so.

**`Size`** wraps a `u64` of bits — a volume of data, where `Bandwidth` is a rate. It mirrors `Bandwidth`'s unit ladders exactly, and closes the triangle between the three: `size.transmission_time(bandwidth)` and `Size::carried_in(bandwidth, duration)` are the same two conversions read from the volume's side, and both are one-line delegations to the `Bandwidth` method that already implements them. Keep it that way — a conversion has one implementation, and it lives on `Bandwidth`. `carried_in` is an associated function rather than a method because it constructs rather than converts; it narrows `bits_in`'s `u128` and reports `None` when that will not fit a `u64`.

`SizeOf { fn size_in_bytes(&self) -> Size }` lives in `size` and is the one thing here the engine implements rather than calls — `Packet` and its parts do, summing header and payload. It returns a `Size`, which is bits-backed, so the method name does not fix the unit the caller reads back.

Note the width asymmetry `Size` inherits from the ladder rule: `from_bytes` takes a `u32` while `as_bytes` returns a `u64`. Building a `Size` from a real buffer length therefore needs a narrowing cast at the call site — `packet.rs` writes `Size::from_bytes(d.len() as u32)` — so the effective ceiling on a payload is that cast at ~4 GB, far below the ~2 EB `Size` itself can hold.

## Conventions

Follow these when adding a method, a unit, or another value type.

- **Fallible by default, `unchecked_*` to opt out.** An operation that can fail returns `Option` under the plain name (`transmission_time`), and the panicking variant carries the `unchecked_` prefix (`unchecked_transmission_time`). The `unchecked_*` fn is defined as `.expect(...)` over the safe one, so there is one implementation. This is the inverse of std's naming and is intentional. `checked_*` / `saturating_*` keep their std meaning for plain arithmetic, where the operator impls are the default path.
- **Each unit derives from the unit below it**, rather than each function carrying its own precomputed multiplier: mega calls kilo, giga calls mega, on both the constructors and the `as_*` accessors. Apply the factor to the *returned value* (`Self::from_kilo_bits_per_sec(bw) * 1_000`), never to the argument — the argument is a `u32` and overflows before it ever widens. `Bandwidth` is the worked example; `SimTime`'s constructors funnel through `from_ps` and `Duration`'s multiply into the field directly, since there the argument cast to `u128` happens first.
- **Two unit ladders on `Bandwidth` and `Size`**, spelled out in words to stay unambiguous: decimal SI (`kilo`/`mega`/`giga`/`tera`, 10³ⁿ) and IEC binary (`kibi`/`mebi`/`gibi`/`tebi`, 2¹⁰ⁿ), each in a bits and a bytes flavour. Both types carry the same ladder under the same names, differing only in the `_per_sec` suffix — a change to one is a change to both.
- **`Display` renders decimal only**, picking the largest unit that fits and printing it to three decimal places, with SI's lowercase `k`: `Bandwidth` over `bps`/`kbps`/`Mbps`/`Gbps`/`Tbps`, `Size` over `b`/`kb`/`Mb`/`Gb`/`Tb`, `Duration` over `ps`/`ns`/`us`/`ms`/`s`. `SimTime` defers to `Duration`, printing `{since_epoch} since epoch`.
- **`as_*` accessors truncate**, which the integer return type already says — do not restate it per method. Only a lossy step the signature doesn't imply (bits→bytes discarding up to 7 bits) or a float accessor keeping the sub-unit part earns a comment.
- Constants over free-standing zero values: `Duration::ZERO`, `Bandwidth::ZERO`, `Size::ZERO`, `SimTime::EPOCH`, `MAX`. `Default` is the zero value and a test asserts it.
- Each type carries the full operator set — `Add`/`Sub`/`Mul`/`Div` with their assign forms, `Sum` over both owned and borrowed iterators, `Div<Self>` returning the truncating ratio — plus `scale(f64)` for inexact derating. Match that surface when adding a type.

## `id`

`IdGenerator` is a `u64` counter: `get_id()` hands out the next value and bumps it. It shares none of the conventions above — no units, no operators, no `Display`, no tests — and it is mutable state rather than a value, so do not reach for the value-type checklist when touching it.

What matters here is what `id.rs` does *not* contain. The typed ids (`NodeId`, `ChannelId`, `PacketId`, …) each live beside the entity they name, and each of those modules adds its own `impl IdGenerator { pub fn new_node_id(&mut self) -> NodeId }` block. That inverts the usual direction on purpose: `id.rs` stays free of any dependency on `core`, and each id's private field never has to leave its own module. Adding an entity means adding its constructor there, not here.

`get_id` bumps with `+=`, so it panics on overflow in debug and wraps in release. Sixty-four bits of ids is not a real ceiling for a simulation run, which is why nothing guards it.

## Style

- Do not write comments for what the type system already expresses. Doc comments earn their place by explaining truncation, panics, `None` cases, units, or a non-obvious choice of width — not by restating the signature.
- Tests are named as a sentence about the behaviour (`each_unit_is_a_thousandfold_of_the_next`, `transmission_time_reports_a_dead_link`). Prefer asserting a property or an edge over re-checking arithmetic that the constructors already guarantee: the ladder tests assert each unit against the one below rather than against a literal, and the interesting tests are the ones covering the dead link, the sub-picosecond truncation, the `u128` overflow, and the volume too wide to narrow back into a `u64`.
