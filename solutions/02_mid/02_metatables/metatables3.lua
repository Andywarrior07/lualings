local RateLimiter = {}
RateLimiter.__tostring = function(self)
	return "RateLimiter(count=" .. self.count .. ")"
end
RateLimiter.__call = function(self)
	return self.count
end

local limiter = setmetatable({ count = 0 }, RateLimiter)
limiter.count = 5

local text = tostring(limiter)
assert(text == "RateLimiter(count=5)", "expected tostring to show the current count, got: " .. text)

local ok, result = pcall(function()
	return limiter()
end)
assert(ok == true, "expected the limiter to be callable")
assert(result == 5, "expected calling the limiter to return the current count, got" .. tostring(result))

_G.__lualings_pass = true
