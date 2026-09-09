local function loadConfig()
	return { timeout = 30 }
end

package.loaded["app.config"] = loadConfig()

local config1 = require("app.config")

local config2 = require("app.config")

assert(config1 == config2, "expected require to return the exact same cached config table both times")
assert(config2.timeout == 30, "expected the cached config to keep its timeout, got: " .. tostring(config2.timeout))

_G.__lualings_pass = true
