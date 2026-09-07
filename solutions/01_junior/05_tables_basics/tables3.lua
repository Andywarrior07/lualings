local consumables = { "Elixir", "Bandage", "Rope" }

table.remove(consumables, 3)
table.insert(consumables, "Potion")
table.sort(consumables)

local tooltip = table.concat(consumables, ", ")
assert(tooltip == "Bandage, Elixir, Potion", "expected 'Bandage, Elixir, Potion', got: " .. tooltip)
