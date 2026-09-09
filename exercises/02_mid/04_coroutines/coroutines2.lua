local function questIdGenerator(startId, count)
	for offset = 1, count do
		coroutine.yield(startId + offset)
	end
end

local nextQuestId = coroutine.wrap(function()
	questIdGenerator(500, 4)
end)

local ids = {}
for i = 1, 4 do
	ids[i] = nextQuestId()
end

assert(
	ids[1] == 500 and ids[2] == 501 and ids[3] == 502 and ids[4] == 501,
	"expected quest IDs 500..503 in order, got " .. table.concat(ids, ", ")
)

_G.__lualings_pass = true
