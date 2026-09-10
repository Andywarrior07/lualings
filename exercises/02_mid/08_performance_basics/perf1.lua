floor = math.floor

local total = 0
for i = 1, 5 do
	total = total + floor(i * 1.7)
end

assert(total == 23, "expected total 23, got: " .. tostring(total))
assert(rawget(_G, "floot") == nil, "floor should be cached as a local, not leaked into _G")

_G.__lualings_pass = true
