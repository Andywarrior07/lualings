local entries = { "Fireball hit for 120", "Frostbolt hit for 95", "Melee hit for 40" }

local pieces = {}
for _, entry in ipairs(entries) do
	table.insert(pieces, entry)
end

local log = table.concat(pieces, ", ")

assert(
	log == "Fireball hit for 120, Frostbolt hit for 95, Melee hit for 40",
	"expected all three entries joined in order, got: " .. log
)

_G.__lualings_pass = true
