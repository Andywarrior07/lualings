local minutes = 3
local seconds = 5

local display = string.format("%d:%02d", minutes, seconds)

assert(display == "3:05", "expected '3:05', got '" .. display .. "'")

_G.__lualings_pass = true
