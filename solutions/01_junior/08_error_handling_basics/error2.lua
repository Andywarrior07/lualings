local function setZoomLevel(value)
	assert(value >= 0 and value <= 2, "zoom must be between 0 and 2")
	return value
end

local ok, err = pcall(setZoomLevel, 5)

assert(ok == false, "expected setting an out-of-range zoom level to fail")
assert(err:find("zoom must be between 0 and 2") ~= nil, "expected the validation message, got: " .. tostring(err))

_G.__lualings_pass = true
