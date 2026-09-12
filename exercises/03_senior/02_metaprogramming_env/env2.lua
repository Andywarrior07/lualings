local sandbox = setmetatable({ print = print }, { __index = _G })

local untrusted_code = "return os ~= nil"
local chunk = load(untrusted_code, "unstristed", "t", sandbox)
local can_reach_os = chunk()

assert(can_reach_os == false, "expected the sandboxed chunk to have no access to os, but it reached it anyway")

_G.__lualings_pass = true
