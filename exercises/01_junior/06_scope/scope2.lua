local function initializeCooldownTracker()
	cooldownRemaining = 30
end

assert(
	rawget(_G, "cooldownRemaining") == nil,
	"cooldownRemaining should not leak into _G event though itś set inside a function"
)

_G.__lualings_pass = true
