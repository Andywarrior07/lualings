local scanCount = 0

local function scanLootValue(raidId)
	scanCount = scanCount + 1
	return raidId * 100
end

local function memoize(fn)
	local cache = {}
	return function(arg)
		if cache[arg] == nil then
			cache[arg] = fn(arg)
		end
		return cache[arg]
	end
end

local cachedScan = memoize(scanLootValue)

cachedScan(42)
cachedScan(42)

assert(
	scanCount == 1,
	"scanLootValue should only run once for the same raidId, got " .. tostring(scanCount) .. " calls"
)

_G.__lualings_pass = true
