local function totalResponseTime(...)
	local total = 0
	for _, value in ipairs({ ... }) do
		total = total + value
	end
	return total
end

local total = totalResponseTime(12, 8, 5)

assert(total == 25, "expected the total across all three stages, got " .. tostring(total))

_G.__lualings_pass = true
