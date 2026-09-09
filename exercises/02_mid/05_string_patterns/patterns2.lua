local query = "zone=Ogrimmar&level=60&class=Warrior"

local params = {}
for key, value in query:gmatch("(%a+)=(%a+)") do
	params[key] = value
end

assert(params.zone == "Ogrimmar", "expected zone 'Ogrimmar', got " .. tostring(params.zone))
assert(params.level == "60", "expected level '60', got " .. tostring(params.level))
assert(params.class == "Warrior", "expected class 'Warrior', got " .. tostring(params.class))

_G.__lualings_pass = true
