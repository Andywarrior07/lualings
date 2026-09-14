# LuaJIT FFI: concepts

This module is **conceptual, not autoevaluated** — see "Why this isn't autoevaluated" below for exactly why, before assuming that's an oversight.

The FFI (Foreign Function Interface) library lets Lua code call C functions and use C data structures directly, by parsing plain C declarations — no separate C module to write or compile, no manual `lua_push*`/`lua_to*` bookkeeping like the C API covered in `04_c_api_concepts`. `ffi1.lua` shows the basic shape of it.

## Basic syntax

```lua
local ffi = require("ffi")

ffi.cdef[[
    int abs(int n);
    typedef struct { double x, y; } point_t;
]]

local n = ffi.C.abs(-42)          -- call a declared C function
local point = ffi.new("point_t", 3, 4)  -- create a cdata instance
print(n, point.x, point.y)
```

- `ffi.cdef[[ ... ]]` takes a block of literal C declarations — function signatures, `struct`s, `typedef`s — and makes them known to LuaJIT, the same way a C header would.
- `ffi.C` is where declared C library functions become callable, as if they were an ordinary Lua table of functions.
- `ffi.new("type", ...)` allocates a `cdata` value of a declared type; its fields (`point.x`, `point.y`) are accessed like a normal Lua table's.
- `ffi.metatype("type", { ... })` (not shown above) attaches Lua metamethods to a `cdata` type, letting `struct`s support operators, `__tostring`, etc.

## Why this matters for performance

Lua's standard C API (the one `04_c_api_concepts` covers) is what LuaJIT itself, and Lua's own standard library, are built on — but LuaJIT's JIT compiler never compiles calls through that C API to machine code; they always go through the slower interpreter path. That's exactly why performance-sensitive OpenResty libraries — `lua-resty-core` (which reimplements part of `ngx_lua`'s own API) and `lua-resty-lrucache` (an LRU cache) — are built on the FFI instead: FFI calls *can* be JIT-compiled, so a hot path written against `ffi.cdef`/`ffi.C` can end up dramatically faster than the equivalent written against the C API.

## Why this isn't autoevaluated

This runner (`mlua`, `lua54` feature, vendored) executes **Lua 5.4**, not LuaJIT — and the `ffi` library is exclusive to LuaJIT; it doesn't exist in standard Lua, at any version. This isn't a gap that a future Lua version closes: `mlua` only allows enabling exactly one Lua version per compiled binary. Verified directly in `mlua-sys`'s build script (not assumed): it emits a hard `compile_error!` — *"You can enable only one of the features: lua55, lua54, lua53, lua52, lua51, luajit, luajit52, luau"* — so a single `lualings` binary can never embed both Lua 5.4 and LuaJIT at once, regardless of how this project's `Cargo.toml` is configured. There's no `.lua`-only way to make `ffi1.lua` real here, and writing a fake "simulated" pass/fail for it would mean inventing an evaluation that checks nothing — same reasoning already applied to `04_c_api_concepts`, not a new exception.

## If this ever gets resolved

A pragmatic alternative has already been identified, but isn't implemented here: shell out to a `luajit` binary already installed on the user's system via `std::process::Command`, and mark the exercise "unavailable" (a clear, distinct state) when no such binary is found — instead of failing in a confusing way that looks like a broken exercise. If that's ever decided, this module moves from "conceptual" to "resolved," following the same process already defined for keeping the project's planning notes up to date — same as the note already left in `04_c_api_concepts`.
