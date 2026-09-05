local healthPercent = 50

local barColor

if healthPercent > 50 then
	barColor = "green"
elseif healthPercent > 20 then
	barColor = "yellow"
else
	barColor = "red"
end

assert(barColor == "yellow", "at exaxctly 50% health the bar should still be yellow, not green")

_G.__lualings_pass = true
