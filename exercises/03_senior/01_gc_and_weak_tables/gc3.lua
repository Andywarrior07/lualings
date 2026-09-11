local closed = false

local mt = {}
local conn = setmetatable({}, mt)
mt.__gc = function()
	closed = true
end

conn = nil
collectgarbage("collect")

assert(
	closed == true,
	"expected the connection's __gc finalizer to have run after ti was released and a collection forced but it never fired"
)

_G.__lualings_pass = true
