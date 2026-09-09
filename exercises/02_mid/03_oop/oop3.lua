local Tooltip = {
	showTooltip = function(self)
		self.tooltipVisible = true
		return self.tooltipVisible
	end,
}

local function applyMixin(class, mixin)
	for key, value in pairs(mixin) do
		-- forgot to actually copy the mixin's function onto the class
	end
end

local Button = {}
Button.__index = Button
applyMixin(Button, Tooltip)

local button = setmetatable({}, Button)

local ok, visible = pcall(function()
	return button:showTooltip()
end)

assert(ok == true, "expected Button to gain showTooltip from the Tooltip mixin")
assert(visible == true, "expected showTooltip to report the tooltip as visible")

_G.__lualings_pass = true
