local function splitCurrency(copper)
	local gold = copper // 100
	local silver = copper % 100
end

local gold, silver = splitCurrency(1250)

assert(gold == 12, "expected 12 gold, got " .. tostring(gold))
assert(silver == 50, "expected 50 silver, got " .. tostring(silver))

_G.__lualings_pass = true
