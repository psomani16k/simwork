# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Scope

`core::util` holds the value types every measurement in the simulator passes through. It is the one complete, heavily tested part of the crate, and is the reference for style everywhere else. Three modules, no submodules: `time` (`SimTime`), `duration` (`Duration`), `bandwidth` (`Bandwidth`).

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

## Conventions

Follow these when adding a method, a unit, or another value type.

- **Fallible by default, `unchecked_*` to opt out.** An operation that can fail returns `Option` under the plain name (`transmission_time`), and the panicking variant carries the `unchecked_` prefix (`unchecked_transmission_time`). The `unchecked_*` fn is defined as `.expect(...)` over the safe one, so there is one implementation. This is the inverse of std's naming and is intentional. `checked_*` / `saturating_*` keep their std meaning for plain arithmetic, where the operator impls are the default path.
- **Each unit derives from the unit below it**, rather than each function carrying its own precomputed multiplier: mega calls kilo, giga calls mega, on both the constructors and the `as_*` accessors. Apply the factor to the *returned value* (`Self::from_kilo_bits_per_sec(bw) * 1_000`), never to the argument — the argument is a `u32` and overflows before it ever widens. `Bandwidth` is the worked example; `SimTime`'s constructors funnel through `from_ps` and `Duration`'s multiply into the field directly, since there the argument cast to `u128` happens first.
- **Two unit ladders on `Bandwidth`**, spelled out in words to stay unambiguous: decimal SI (`kilo`/`mega`/`giga`/`tera`, 10³ⁿ) and IEC binary (`kibi`/`mebi`/`gibi`/`tebi`, 2¹⁰ⁿ), each in a bits and a bytes flavour. `Display` renders decimal only — `bps`, `kbps`, `Mbps`, `Gbps`, `Tbps` — with SI's lowercase `k`, picking the largest unit that fits. `Duration`'s `Display` does the same over `ps`/`ns`/`us`/`ms`/`s`.
- **`as_*` accessors truncate**, which the integer return type already says — do not restate it per method. Only a lossy step the signature doesn't imply (bits→bytes discarding up to 7 bits) or a float accessor keeping the sub-unit part earns a comment.
- Constants over free-standing zero values: `Duration::ZERO`, `Bandwidth::ZERO`, `SimTime::EPOCH`, `MAX`. `Default` is the zero value and a test asserts it.
- Each type carries the full operator set — `Add`/`Sub`/`Mul`/`Div` with their assign forms, `Sum` over both owned and borrowed iterators, `Div<Self>` returning the truncating ratio — plus `scale(f64)` for inexact derating. Match that surface when adding a type.

## Style

- Do not write comments for what the type system already expresses. Doc comments earn their place by explaining truncation, panics, `None` cases, units, or a non-obvious choice of width — not by restating the signature.
- Tests are named as a sentence about the behaviour (`each_unit_is_a_thousandfold_of_the_next`, `transmission_time_reports_a_dead_link`). Prefer asserting a property or an edge over re-checking arithmetic that the constructors already guarantee: the ladder tests assert each unit against the one below rather than against a literal, and the interesting tests are the ones covering the dead link, the sub-picosecond truncation, and the `u128` overflow.
