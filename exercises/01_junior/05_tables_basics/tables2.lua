local abilityBar = { "fireball", "heal", nil, "dash" }
local equippedCount = #abilityBar

assert(equippedCount == 3, "expected 3 equipped abilities, got " .. equippedCount)

_G.__lualings_pass = true
