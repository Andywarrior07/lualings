local function computeDamage(base, multiplier)
	return base * multiplier.value
end

local function errorHandler(err)
	return err
end

local ok, result = xpcall(computeDamage, errorHandler, 20, nil)

assert(ok == false, "expected the risky call to fail, ok=" .. tostring(ok) .. " result=" .. tostring(result))
assert(
	result:find("^combat: ") ~= nil,
	"expected the handler to prefix the error with 'combat: ', got: " .. tostring(result)
)

_G.__lualings_pass = true
