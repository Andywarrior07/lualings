local attempts = 0
local maxAttempts = 3

repeat
	attempts = attempts + 1
until attempts < maxAttempts

assert(attempts == 3, "expected exactly 3 attempts, got " .. attempts)

_G.__lualings_pass = true
