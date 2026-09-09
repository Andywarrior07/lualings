local realConstants = { critMultiplier = 2.0, baseHitChange = 0.95 }

local readOnlyConstants = setmetatable({}, {
	__index = realConstants,
	__newindex = function(t, key, value)
		error("attempt to modify read-only table: " .. tostring(key))
	end,
})

local ok = pcall(function()
	readOnlyConstants.critMultiplier = 99
end)

assert(ok == false, "expected writing to the read-only proxy to fail")
assert(realConstants.critMultiplier == 2.0, "the real constants table should stay untouched")

_G.__lualings_pass = true
