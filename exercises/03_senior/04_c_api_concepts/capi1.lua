--[[
-- This module is conceptual: this runner is Lua 5.4 vendored inside
-- mlua, with no C compile wired up to build a real `mylib.so` for it to load.
-- There's nothing here to run, and nothing for you to fix.
-- Read README.md instead.
-- ]]

local mylib = {
	fast_sum = function(a, b)
		return a + b
	end,
}

local total = mylib.fast_sum(2, 3)
print(total)
