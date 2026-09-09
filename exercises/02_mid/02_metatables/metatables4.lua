local realConstants = { critMultiplier = 2.0, baseHitChance = 0.95 }

local readOnlyConstants = setmetatable({}, {
	__index = realConstants,
	__newindex = function(t, key, value)
		-- forgot to block the write
	end,
})

local ok = pcall(function()
	readOnlyConstants.critMultiplier = 99
end)

assert(ok == false, "expected writing to the read-only proxy to fail")
assert(realConstants.critMultiplier == 2.0, "the real constants table should stay untouched")
