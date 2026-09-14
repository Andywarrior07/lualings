# The Lua C API: concepts

This module is **conceptual, not autoevaluated** — see "Why this isn't autoevaluated" below for exactly why, before assuming that's an oversight.

Lua's C API is how a host program (LÖVE2D's engine, Redis, `nginx`/OpenResty, WoW's client) embeds Lua and exposes native functionality to Lua scripts. `capi1.lua` shows what that looks like from the Lua side: a table of functions that happen to be implemented in C, called exactly like any other Lua table of functions. This README covers the C side that makes that possible.

## `lua_State *L`

Every C function registered with Lua receives a `lua_State *L` — the interpreter's entire state: its stack, globals, loaded libraries, GC bookkeeping. A C function's signature always looks like this:

```c
static int l_fast_sum(lua_State *L) {
    /* ... */
    return 1; /* number of return values pushed onto the stack */
}
```

`L` is opaque from C's side — you never read its fields directly, only call `lua_*`/`luaL_*` functions on it.

## The stack

The C API doesn't pass Lua values as C function arguments/return values directly — everything goes through `L`'s stack. When Lua calls `l_fast_sum(a, b)`, the arguments are already sitting on the stack before your C function runs; you read them by *position*, not by receiving them as C parameters:

```c
static int l_fast_sum(lua_State *L) {
    double a = lua_tonumber(L, 1); /* first argument */
    double b = lua_tonumber(L, 2); /* second argument */

    lua_pushnumber(L, a + b);      /* push the result */
    return 1;                      /* "I left 1 value on the stack" */
}
```

The return value of the C function is a plain `int`: how many values it pushed onto the stack as results, not the values themselves. This stack-based convention is why the C API needs so much explicit bookkeeping compared to, say, calling a function pointer directly — there's no shared type system between C and Lua, so every value has to cross that boundary through explicit `lua_push*`/`lua_to*` calls.

## Registering a C function

A single function can be pushed and bound to a global directly:

```c
lua_pushcfunction(L, l_fast_sum);
lua_setglobal(L, "fast_sum");
```

A whole module (the `mylib` from `capi1.lua`) is normally registered as a table of functions via a `luaL_Reg` array and a `luaopen_*` entry point — the same shape the standard library itself uses internally:

```c
static const luaL_Reg mylib_functions[] = {
    {"fast_sum", l_fast_sum},
    {NULL, NULL} /* sentinel, marks the end of the array */
};

int luaopen_mylib(lua_State *L) {
    luaL_newlib(L, mylib_functions);
    return 1; /* leaves the new module table on the stack */
}
```

When a script does `local mylib = require("mylib")`, this is the function `require` ends up calling.

## Why this isn't autoevaluated

The C API is exercised from C (or from Rust against `mlua`'s own API, which is the pragmatic alternative noted below) — never from a `.lua` file, regardless of which Lua version the interpreter is. This runner (`mlua`, `lua54` feature, vendored) can execute Lua 5.4 scripts, but it has no C compiler wired into the exercise pipeline to build a `mylib.so`/`mylib.dll` from a `luaopen_mylib` for `capi1.lua` to actually `require`. There's no `.lua`-only way to make this real — writing a fake "simulated" pass/fail for it would mean inventing an evaluation that checks nothing, which is worse than being upfront that this content is explanatory only. This is a structural limitation of the runner, not a TODO to burn down with more content-writing effort.

## If this ever gets resolved

A pragmatic alternative has already been identified, but isn't implemented here: reformulate as exercises written in **Rust**, compiled via `cargo test`, exercising `mlua`'s own embedding API (`Lua::create_function`, etc.) as a memory-safe equivalent of the raw C API concepts above — same `lua_State`/stack ideas, but through `mlua`'s Rust bindings instead of raw `lua_*` C calls. If that's ever decided, this module moves from "conceptual" to "resolved," following the same process already defined for keeping the project's planning notes up to date.
