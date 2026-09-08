local requestCount = 0

local function makeRouteCounter()
	return function()
		requestCount = requestCount + 1
		return requestCount
	end
end

local usersRouteCounter = makeRouteCounter()
local ordersRouteCounter = makeRouteCounter()

usersRouteCounter()
usersRouteCounter()
local ordersCount = ordersRouteCounter()
assert(
	ordersCount == 1,
	"the /orders route counter should start at 1, indepennndent of /users request - got " .. tostring(ordersCount)
)

_G.__lualings_pass = true
