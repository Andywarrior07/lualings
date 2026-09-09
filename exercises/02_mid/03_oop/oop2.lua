local Spell = {}
Spell.__index = Spell

function Spell.new(name)
	return setmetatable({ name = name }, Spell)
end

function Spell:describe()
	return self.name .. " is a spell"
end

local FireSpell = {}
FireSpell.__index = FireSpell

function FireSpell.new(name, damage)
	return setmetatable({ name = name, damage = damage }, FireSpell)
end

local function isInstanceOf(instance, class)
	local mt = getmetatable(instance)

	while mt ~= nil do
		if mt == class then
			return true
		end
		mt = getmetatable(mt) and getmetatable(mt).__index
	end
	return false
end

local fireball = FireSpell.new("Fireball", 50)

local ok, description = pcall(function()
	return fireball:describe()
end)

assert(ok == true, "expected FireSpell to inherit describe() from spell")
assert(
	description == "Fireball is a spell",
	"expected the inherited describe() to work, got: " .. tostring(description)
)
assert(isInstanceOf(fireball, Spell) == true, "expected fireball to be a spell via the inheritance chain")

_G.__lualings_pass = true
