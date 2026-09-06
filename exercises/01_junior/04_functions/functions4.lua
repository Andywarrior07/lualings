local RANK_POINTS = { [1] = 500, [2] = 300, [3] = 150 }

local function totalPointsThroughRank(rank)
	return RANK_POINTS[rank] + totalPointsThroughRank(rank - 1)
end

local total = totalPointsThroughRank(3)

print(total)
