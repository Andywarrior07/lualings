--[[
-- This module is conceptual: this runner is Lua 5.4 vendored inside mlua, not LuaJIT, and mlua
-- only allows enabling one Lua version per compiled binary (lua54/lua53/luajit/etc. are mutually
-- exclussive), the `ffi` library doesn't exist outside LuaJIT, regardless of Lua version.
-- There's nothing here to run and nothing for you to fix, read README.md instead.
-- ]]

local ffi = require("ffi")

ffi.cdef([[
    int abs(int n);
    typedef struct { double x, y; } point_t;
  ]])

local n = ffi.C.abs(-42)

local point = ffi.new("point_t", 3, 4)
print(n, point.x, point.y)
