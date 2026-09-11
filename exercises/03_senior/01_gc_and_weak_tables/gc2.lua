local cache = {}

local session = { id = "abc123" }
cache[session] = { hits = 1 }

session = nil
collectgarbage("collect")

assert(
	next(cache) == nil,
	"expected the cache entry to be gone after the session was released and a collection forced, but it's still there"
)

_G.__lualings_pass = true
