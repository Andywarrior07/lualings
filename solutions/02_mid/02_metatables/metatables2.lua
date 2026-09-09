local defaults = { jump = "space", dash = "shift" }
local overrides = {}
local keybinds = setmetatable({}, {
	__index = defaults,
	__newindex = function(t, key, value)
		overrides[key] = value
	end,
})

keybinds.jump = "w"

assert(overrides.jump == "w", "expected the rebind to be stored in overrides, got" .. tostring(overrides.jump))
assert(defaults.jump == "space", "defaults should stay untouched")

_G.__lualings_pass = true
