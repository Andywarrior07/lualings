local assert = assert
local rawget = rawget
local real_G = _G

do
	_ENV = { print = print }
	local calibration = 21 * 2
	print(calibration)
end

score = 42

assert(
	rawget(real_G, "score") == 42,
	"expected 'score' to land in the real global table after the calibration block ended, but the block's private _ENV leakded past it"
)

_G.__lualings_pass = true
