local consumables = { "Elixir", "Bandage", "Rope" }

table.remove(consumables, 1)
table.insert(consumables, "Potion")
table.sort(consumables)

local tooltip = table.concat(consumables, ", ")

assert(tooltip == "Bandage, Elixir, Potion", "expected 'Bandage, Elixir, Potion', got: " .. tooltip)

_G.__lualings_pass = true
