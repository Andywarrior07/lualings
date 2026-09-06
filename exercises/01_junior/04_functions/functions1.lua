local function calculateDamage(basePower, multiplier)
	return basePower * multiplier
end

local damage = calculateDamage(20)

print(damage)
