local RANK_POINTS = { [1] = 500, [2] = 300, [3] = 150 }

local function totalPointsThroughRank(rank)
	if rank < 1 then
		return 0
	end
	return RANK_POINTS[rank] + totalPointsThroughRank(rank - 1)
end

local total = totalPointsThroughRank(3)

print(total)
