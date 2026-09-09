local message = "Selling epic gear for 5000g, contact Bob92"

local redacted, count = message:gsub("%d", "#")

assert(
	redacted == "Selling epic gear for ####g, contact Bob##",
	"expected all digits replaced with '#', got: " .. redacted
)
assert(count == 6, "expected 6 digits replaced, got: " .. count)

_G.__lualings_pass = true
