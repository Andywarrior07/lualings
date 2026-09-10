local function equipItem(itemId, slot)
	if slot > 19 then
		error("invalid slot: " .. slot)
	end
	return true
end

local ok, err = pcall(equipItem, 12345, 25)

assert(ok == false, "expected equipping an out-of-range slot to fail")
assert(type(err) == "table", "expected the error to be a structured table, got a " .. type(err))
assert(err.code == "INVALID_SLOT", "expected err.code to be 'INVALID_SLOT', got: " .. tostring(err))
assert(err.message == "invalid slot: 25", "expected err.message to describe the problem, got: " .. tostring(err))

_G.__lualings_pass = true
