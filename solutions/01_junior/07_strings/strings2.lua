local playerName = "José"

local nameLength = utf8.len(playerName)

assert(nameLength == 4, "expected 4 characters in 'José', got " .. nameLength)

_G.__lualings_pass = true
