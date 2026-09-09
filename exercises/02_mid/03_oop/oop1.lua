local Entity = {}
Entity.__index = Entity

function Entity.new(name, health)
	local self = { name = name, health = health }

	return self
end

function Entity:takeDamage(amount)
	self.health = self.health - amount
	return self.health
end

local goblin = Entity.new("Goblin", 30)

local ok, remaining = pcall(function()
	return goblin:takeDamage(10)
end)

assert(ok == true, "expected goblin:takeDamage to be callable through the Entity class")
assert(remaining == 20, "expected 20 health remaining after taking 10 damage, got " .. tostring(remaining))

_G.__lualings_pass = true
