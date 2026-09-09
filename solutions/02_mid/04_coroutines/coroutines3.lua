local function applyBuffSteps()
	coroutine.yield("armor")
	coroutine.yield("resistance")
end

local buffTask = coroutine.create(applyBuffSteps)

local statuses = {}
statuses[1] = coroutine.status(buffTask)

coroutine.resume(buffTask)
statuses[2] = coroutine.status(buffTask)

coroutine.resume(buffTask)
coroutine.resume(buffTask)
statuses[3] = coroutine.status(buffTask)

assert(statuses[1] == "suspended", "expected the freshly created task to be suspended, got " .. tostring(statuses[1]))
assert(
	statuses[2] == "suspended",
	"expected the task to still be suspended between yields, got " .. tostring(statuses[2])
)
assert(statuses[3] == "dead", "expected the task to be dead once it fully ran, got " .. tostring(statuses[3]))

local rollDamage = coroutine.wrap(function()
	coroutine.yield(10)
	coroutine.yield(15)
end)

local firstRoll = rollDamage()
local secondRoll = rollDamage()

assert(firstRoll == 10, "expected the first roll to be 10, got " .. tostring(firstRoll))
assert(secondRoll == 15, "expected the second roll to be 15, got " .. tostring(secondRoll))

_G.__lualings_pass = true
