local attempts = 0
local succeeded = false

::retry::
attempts = attempts + 1
if attempts == 3 then
  succeeded = true
elseif attempts < 3 then
  goto retry
end

assert(succeeded == true, "expected the retry loop to eventually succeed")
assert(attempts ==3 , "expected exactly 3 attempts, got: " .. attempts)

local total_items = 25
local page_size = 10
local pages = total_items // page_size

assert(pages == 2, "expected 25 items in pages of 10 to need floor(25/10) = 2 full pages, got: " .. pages)

local config_snippet = "local MAX_RETRIES = 3\nMAX_RETRIES = 99"
local chunk, err = load(config_snippet)

assert(chunk == nil, "expected reassigning MAX_RETRIES to fail to compile, but it compiled fine")
assert(err ~= nil, "expected a compile error message when reassigning MAX_RETRIES")

_G.__lualings_pass = true
