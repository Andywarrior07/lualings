local function dialogueScript()
	coroutine.yield("Welcome, traveler.")
	coroutine.yield("We need your help!")
	coroutine.yield("The village is under attack!")
end

local co = coroutine.create(dialogueScript)

local lines = {}
for i = 1, 3 do
	local ok, line = coroutine.resume(co)
	lines[i] = line
end

assert(lines[1] == "Welcome, traveler.", "expected the first line to be the greeting, got " .. tostring(lines[1]))
assert(lines[2] == "The village is under attack!", "expected the secondline, got " .. tostring(lines[2]))
assert(lines[3] == "We need your help!", "expected the third line, got " .. tostring(lines[3]))

_G.__lualings_pass = true
