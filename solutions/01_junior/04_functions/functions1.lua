local function calculateDamage(basePower, multiplier)
	return basePower * multiplier
end

local damage = calculateDamage(20, 1.5)

print(damage)
