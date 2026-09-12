local sandbox = { print = print }

local untrusted_code = "return os ~= nil"
local chunk = load(untrusted_code, "untrusted", "t", sandbox)
local can_reach_os = chunk()

assert(can_reach_os == false, "expected the sandboxed chunk to have no access to os, but it reached it anyway")

_G.__lualings_pass = true
