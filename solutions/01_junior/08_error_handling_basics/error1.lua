local addonSettings = {}

local ok, err = pcall(function()
	return addonSettings.profile.zoom
end)

assert(ok == false, "expected the protected call to fail because profile is missing")
assert(err:find("profile") ~= nil, "expected the error to mention 'profile', got: " .. tostring(err))

_G.__lualings_pass = true
