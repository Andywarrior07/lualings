local function makeJumpCounter()
	return function()
		local jumps = 0
		jumps = jumps + 1
		return jumps
	end
end

local countJumps = makeJumpCounter()

countJumps()
countJumps()
local total = countJumps()

assert(total == 3, "expected 3 jumps counted across all calls, got " .. tostring(total))

_G.__lualings_pass = true
