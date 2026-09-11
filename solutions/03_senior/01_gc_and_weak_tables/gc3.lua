local closed = false

local conn = setmetatable({}, {
	__gc = function()
		closed = true
	end,
})

conn = nil
collectgarbage("collect")

assert(
	closed == true,
	"expected the connection's __gc finalizer to have run after it was released and a collection forced but it never fired"
)

_G.__lualings_pass = true
