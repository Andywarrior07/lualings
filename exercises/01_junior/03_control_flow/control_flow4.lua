local savedData = { 1001, 1002, 1003, trackingEnabled = true, sortOrder = "asc" }

local questCount = 0

for _ in pairs(savedData) do
	questCount = questCount + 1
end

assert(questCount == 3, "expected exactly 3 tracked quest IDs, got " .. questCount)

_G.__lualings_pass = true
