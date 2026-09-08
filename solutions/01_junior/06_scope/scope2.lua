local function initializeCooldownTracker()
	local cooldownRemaining = 30
end

initializeCooldownTracker()

assert(
	rawget(_G, "cooldownRemaining") == nil,
	"cooldownRemaining should not leak into _G even though it's set inside a function "
)
