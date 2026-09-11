local debris = {}
for i = 1, 50000 do
	debris[i] = { x = i, y = i * 2, ttl = 3.0 }
end

local peak = collectgarbage("count")
debris = nil
collectgarbage("collect")

local after = collectgarbage("count")

assert(
	after < peak - 100,
	"expected memory to drop by more than 100KB after releasing debris, got peak=" .. peak .. " after=" .. after
)

_G.__lualings_pass = true
