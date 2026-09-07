local abilityBar = { "fireball", "heal", nil, "dash" }

local equippedCount = 0
for i = 1, 4 do
	if abilityBar[i] ~= nil then
		equippedCount = equippedCount + 1
	end
end

assert(equippedCount == 3, "expected 3 equipped abilities, got " .. equippedCount)

_G.__lualings_pass = true
