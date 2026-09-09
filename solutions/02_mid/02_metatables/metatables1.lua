local defaults = { scale = 1.0, alpha = 0.8, locked = false }

local frameSettings = setmetatable({ scale = 1.2, { __index = defaults } })
local alpha = frameSettings.alpha

assert(alpha == 0.8, "expected alpha to fall back to the default 0.8, got " .. tostring(alpha))

_G.__lualings_pass = true
