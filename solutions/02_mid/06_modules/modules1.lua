local M = {}

function M.trim(s)
	return s:match("^%s*(.-)%s*$")
end

function M.isBlank(s)
	return M.trim(s) == ""
end

assert(
	M.trim("  hello  ") == "hello",
	"expected M.trim to strip surrounding whitespace, got: '" .. tostring(M.trim("  hello  ")) .. "'"
)
assert(rawget(_G, "isBlank") == nil, "isBlank should not leak into _G, define it as M.isBlank instead")

_G.__lualings_pass = true
return M
