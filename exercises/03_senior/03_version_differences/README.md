# Lua version differences: 5.1 → 5.4, and LuaJIT

This project's runner (`mlua`, `lua54` feature, vendored) always executes **Lua 5.4**. `versions1.lua` can only exercise real 5.4 features (`goto`, `//`, `<const>`) — everything below about other versions is explanatory, not something you can run here. It's the same limitation already documented for `04_c_api_concepts` and `05_luajit_ffi`, just less severe: this module's exercise is fully autoevaluable, only the cross-version comparison itself isn't.

| Feature | 5.1 | 5.2 | 5.3 | 5.4 | LuaJIT |
|---|---|---|---|---|---|
| `goto` / labels | no | **introduced** | yes | yes | yes (backported from 5.2) |
| `_ENV` (replaces `setfenv`/`getfenv`) | no (`setfenv`/`getfenv`) | **introduced** | yes | yes | no — kept on the 5.1 model for ABI compatibility |
| `bit32` library | no | **introduced** | present but deprecated (`LUA_COMPAT_BIT32`) | **removed** | no — has its own separate `bit.*` library instead |
| Native bitwise operators (`&`, `\|`, `~`, `<<`, `>>`) | no | no | **introduced** (64-bit integers) | yes | no (uses `bit.*`, not native operators) |
| `//` (floor division) | no | no | **introduced** | yes | no |
| `<const>` / `<close>` attributes | no | no | no | **introduced** | no |

## Notes

- **`bit32`'s lifecycle**: introduced in 5.2, explicitly deprecated in 5.3 once native bitwise operators arrived (kept around only via the `LUA_COMPAT_BIT32` compile flag for gradual migration), fully removed in 5.4. The reason it wasn't just kept alongside the new operators: `bit32` operates on 32-bit integers, while the native operators work on Lua's full 64-bit integer subtype (also new in 5.3) — the two don't agree on wide values, so carrying both indefinitely would mean two subtly incompatible ways to do bitwise math.
- **LuaJIT is not "Lua 5.1 forever"**: it backports a handful of 5.2 syntax additions (`goto`/labels, hex/`\z` escapes, `string.rep`'s separator argument) and a few standalone 5.3 functions (`table.move`, `coroutine.isyieldable`, `\u{...}` escapes), but it never adopted `_ENV`, the native bitwise operators, `//`, or the 5.4 attributes — those depend on runtime changes LuaJIT's Lua-5.1-compatible ABI doesn't have.
- **Why `<const>` needs `load(...)` to test, not a top-level reassignment**: reassigning a `<const>` variable is a **compile-time** error, not a runtime one. A `.lua` exercise file has to be valid Lua 5.4 end to end for this runner to even load it — a real reassignment at the top level would make the whole file fail with a `SyntaxError` before any exercise-specific check ran. Testing it for real means building the offending code as a **string** and compiling it at runtime with `load(...)`, then checking that compilation itself failed (`load` returns `nil` plus an error message instead of raising).
