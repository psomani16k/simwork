# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Scope

`core` is the simulation engine: the five layers, the events that move work
between them, and the loop that drives it. `core::util` under it holds the value
types everything here measures with and is documented separately — read
`util/CLAUDE.md` before touching those.

The application layer is built. The four below it are still scaffolding. What
follows describes the shape all five are meant to share, so that finishing one
is a matter of following the pattern rather than inventing a new one.

## The loop

`Sim` owns one `HashMap` per layer, an `IdGenerator`, and an `EventQueue`.
`Sim::run` pops events in order, advances `now` to each event's timestamp, skips
cancelled ones and dispatches the rest.

`EventQueue` is a `BTreeSet<EventSortKey>` beside a `HashMap<EventId, Event>`.
The key sorts chronologically and breaks ties on `EventId`; since ids come from a
single counter that is submission order, so events scheduled for the same instant
run in the order they were created. Several things below depend on that, so
`EventSortKey`'s `PartialOrd` delegates to its hand-written `Ord` rather than
being derived — a derive would compare `uid` first and silently disagree.

Cancellation is a flag on the `Event`, not a removal: `cancel_event` finds it by
id and marks it, and `run` skips it on the way past. Cancelling something already
popped is a no-op.

## Layer isolation

Each layer is a pair: a `Box<dyn XImpl>` holding whatever that layer's behaviour
needs, wrapped in a struct holding what the *engine* needs to route to it. An
implementation is handed an `XContext` and nothing else. That context is the
whole of its reach, so the boundary between layers is a matter of what types
appear in the signatures rather than a rule to remember.

`ApplicationImpl` is the worked example. Its vocabulary is bytes, `SocketId`,
`Endpoint`, `Size` and `TimerId` — there is no way to name a `Packet`, a header,
a `NodeId` or a `DeviceId` from inside one. `ApplicationEventData` is narrowed to
match, which is why it carries `Data(Vec<u8>)` rather than a packet.

Two required methods, `on_start` and `on_data`; every other callback defaults to
a no-op, so a traffic generator implements two things. Defaults discard their
arguments with `let _ = (..)` rather than `_`-prefixing them, so the parameter
names still read as documentation.

**Cross-layer calls are always events, never direct calls.** A context method
never reaches into the neighbouring entity. This buys no re-entrancy — a socket
cannot call back into the application that is mid-`on_data` — no borrow
gymnastics inside `Sim`, and a causal record of every layer transition in the
queue. It is also what makes processing delay expressible at all.

## The outbox, and where delay goes

A context does not schedule the events it produces. It pushes them into an
outbox — for the application layer, a `Vec<(SocketId, SocketEvent)>` lent by
`Sim` and reused across dispatches. `Sim` drains it after the implementation
returns, stamps each entry with an arrival time and enqueues it.

That indirection exists for one reason: **implementations describe behaviour,
`Sim` describes what behaviour costs.** An implementation that scheduled its own
work would hard-code its own delays, and could not then be run under a different
cost model or with delays off.

`Sim::cross_layer_arrival` is the single seam that stamp goes through. It
currently returns `now`, which the queue's submission-order tie-break keeps
correct. When a real model lands it belongs there and nowhere else. Two
constraints on whatever goes in:

- **It must be FIFO-preserving.** A delay that varies with the work — a cost per
  byte, say — can otherwise reorder two segments handed down microseconds apart.
  Between two layers inside one node that is not realism, it is a bug: a
  transport implementation would observe reordering a real stack cannot produce.
- **Latency and occupancy are different models.** Charging a flat transit time
  gives every entity infinite parallelism and mostly just shifts timestamps.
  Charging occupancy — a `busy_until` per node, arrival at
  `max(now, busy_until) + service` — is what produces queueing, which is the
  only reason processing delay is interesting. It also preserves FIFO for free.

## Timers

A timer is an event an entity addresses back at itself. `TimerId` wraps the
`EventId`, so cancelling a timer is cancelling that event and `cancel_timer` is
`EventQueue::cancel_event`.

Timers bypass the outbox and go straight to the queue. That is the general rule
made concrete: **a self-addressed event crosses no layer and so costs nothing.**
The entity named the instant it wants to wake at, and charging it a handoff
delay would move that instant. A cost model keyed on the source and destination
layers gets this right by construction.

## Flow control runs in both directions

The two return values are the load-bearing part of the design, and they are
symmetric:

- app → socket: `Send { bytes }`, answered by `Sent { accepted }`
- socket → app: `Data(bytes)`, answered by `Consumed { bytes }`

`ApplicationImpl::on_data` returns how much it took; `Application::deliver`
turns that into the `Consumed` reply without the implementation doing anything,
and clamps an over-report to what was actually delivered. Whatever is left stays
in the socket's receive buffer, which is what holds the receive window shut. A
consumed count of zero is still sent — a stalled reader is information the
transport layer needs.

The same shape going the other way lets a windowed transport and a
fire-and-forget one sit behind one trait with the same application above them:
neither side has to know the other's buffering strategy.

Because a handoff costs time, a short write cannot be discovered synchronously.
`on_writable` therefore carries an `available: Size` that is a **credit to
spend**, not an invitation to ask again — the answer to a fresh question would
be stale by the time it arrived. This is the same reason NIC descriptor rings
and TCP's send window work the way they do.

**Buffers belong to the impl, not the wrapper.** `Socket` should hold only what
routing needs. Send and receive buffers are transport state; the receive buffer
*is* the window.

## Ownership and lifetime

`Application` holds a `HashSet<SocketId>` and keeps it in step itself:
`Accepted` inserts *before* the callback, so an implementation can write to the
new socket from inside `on_accepted`; `Closed` removes after.

`Sim::drain_to_sockets` asserts that an application only ever acts on a socket
it owns. Without it a forged `SocketId` would let one application drive
another's transport state, which is a boundary violation the type system cannot
catch.

Events addressed to an entity that is already gone are **dropped, not fatal**.
Once a handoff costs time an entity can be torn down while work aimed at it is
still in flight, and that is ordinary. Keep the `if let Some(..)` shape rather
than reaching for `expect`. Cancellation is reserved for timers, where the
entity holds the id and genuinely owns the event.

Panics are for routing bugs only — `source_socket` panics when an application
event arrives from something that is not a socket, because the engine has
misrouted it.

## Where the layers stand

- **Application** — done. `ApplicationImpl`, `ApplicationContext`, `Application`.
- **Transport** — `SocketImpl` is still an empty trait. `SocketEvent` has its
  down-calls from the application, its `Consumed` reply and its timers, but the
  inbound path from the network layer is still `Packet(Packet)` and is the next
  leak to close.
- **Network, link, physical** — field-only structs. `Node` has no routing,
  `Device` has no MTU, `Channel::new` has no id generation, and the matching
  arms in `Sim::handle_event` are `todo!()`.

The transport layer is what to build next, and the shape of it follows from the
above. `SocketImpl` should speak bytes upward and a transport PDU downward:
split `Header` into `TransportHeader { Tcp, Udp }` and `NetworkHeader
{ Ipv4, Ipv6 }`, and give the socket a `Segment { TransportHeader, payload }`
that the node wraps into a `Packet`. That turns "a socket must not build an IP
header" from a convention into a compile error, the same way
`ApplicationEventData` does one layer up. `SocketContext::transmit` should take
an `Endpoint` — the socket names a peer, never a route, a device or a channel.

Two things a transport implementation will need that do not exist yet: an MSS,
which originates at `Device` and should reach the socket as a number on the
context rather than as a device to ask; and a demux table mapping an inbound
4-tuple to a `SocketId`, which belongs to `Sim` or `Node` and never to a socket
implementation.

## Conventions

- Ids are `pub struct XId(u64)` beside the entity they name, with the
  constructor added as an `impl IdGenerator` block in that same module — see
  `util/CLAUDE.md` for why the direction is inverted. They all carry
  `Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug`.
- Do not write comments for what the type system already expresses. The doc
  comments that earn their place here are the ones explaining why a boundary is
  where it is, what a count short of the whole means, or which of two plausible
  models a piece of code implements.
- Tests live in the same file under `#[cfg(test)] mod tests` and are named as a
  sentence about the behaviour. `application.rs` drives `Application` directly
  through a small harness holding the `IdGenerator`, `EventQueue` and outbox
  that `Sim` would otherwise lend it — a layer should be testable without the
  layer beneath it existing, and if it is not, the boundary is wrong.
