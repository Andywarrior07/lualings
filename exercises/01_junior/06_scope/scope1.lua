killCount = 0
killCount = killCount + 1
killCount = killCount + 1

assert(rawget(_G, "killCount") == nil, "killCount should not leak into _G")

_G.__lualings_pass = true
