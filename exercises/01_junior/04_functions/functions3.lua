local function totalResponseTime(...)
	local first, second = ...
	return first + second
end

local total = totalResponseTime(12, 8, 5)

assert(total == 25, "expected the total across all three stages, got " .. tostring(total))

_G.__lualings_pass = true
