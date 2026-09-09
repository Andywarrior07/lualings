local command = "/cast Fireball 3"
local spellName, rank = command:match("/cast (%a+) %d+")

assert(spellName == "Fireball", "expected spell name 'Fireball', got " .. tostring(spellName))
assert(rank == "3", "expected rank '3', got " .. tostring(rank))

_G.__lualings_pass = true
