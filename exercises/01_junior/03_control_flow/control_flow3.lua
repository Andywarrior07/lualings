local maxAttempts = 3
local totalWaitSeconds = 0

for attempt = 1, maxAttempts - 1 do
	totalWaitSeconds = totalWaitSeconds + attempt
end

assert(totalWaitSeconds == 6, "expected 1 + 2 + 3 seconds of total backof, got " .. totalWaitSeconds)

_G.__lualings_pass = true
