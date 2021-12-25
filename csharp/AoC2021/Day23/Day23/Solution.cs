using System.Diagnostics.CodeAnalysis;

namespace Day23;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var (amphipods, sideRoomDepth) = Parse(input);
        var cheapestPath = FindCheapestPath(amphipods, sideRoomDepth);
        if (cheapestPath.HasValue)
            Console.WriteLine($"The cheapest solution has cost {cheapestPath}");
        else
            Console.Error.WriteLine("No solution exists.");
    }

    public static int? FindCheapestPath(IEnumerable<Amphipod> amphipods, int sideRoomDepth)
    {
        var queue = new PriorityQueue<Burrow, int>();
        var costs = new Dictionary<Burrow, int>();

        var initialState = new Burrow(amphipods.ToList(), sideRoomDepth);
        queue.Enqueue(initialState, 0);
        costs[initialState] = 0;

        int? cheapest = null;

        while (queue.TryDequeue(out var burrow, out _))
        {
            var costSoFar = costs[burrow];
            if (costSoFar >= cheapest)
                continue;

            if (burrow.TryFindMoveToSideRoom(out var nextStateSr, out var costSr))
            {
                ProcessNextState(
                    nextStateSr,
                    costSoFar + costSr,
                    ref queue,
                    ref costs,
                    ref cheapest
                );
            }
            else
            {
                foreach (var (nextState, cost) in burrow.FindMovesToHallway())
                {
                    ProcessNextState(
                        nextState,
                        costSoFar + cost,
                        ref queue,
                        ref costs,
                        ref cheapest
                    );
                }
            }
        }

        return cheapest;
    }

    private static void ProcessNextState(
        Burrow nextState,
        int cost,
        ref PriorityQueue<Burrow, int> queue,
        ref Dictionary<Burrow, int> costs,
        ref int? cheapest
    )
    {
        if (cost + nextState.MinimumRemainingCost() >= cheapest)
            return;

        if (nextState.IsFinished())
        {
            cheapest = cost;
        }
        else if (costs.ContainsKey(nextState))
        {
            if (cost < costs[nextState])
            {
                queue.Enqueue(nextState, nextState.ComputePriority());
                costs[nextState] = cost;
            }
        }
        else
        {
            queue.Enqueue(nextState, nextState.ComputePriority());
            costs[nextState] = cost;
        }
    }

    public static (IEnumerable<Amphipod> Amphipods, int SideRoomDepth) Parse(IEnumerable<string> input)
    {
        var amphipods = new List<Amphipod>();
        var minDepth = 0;
        var maxDepth = 0;

        var y = 0;
        foreach (var line in input)
        {
            y += 1;
            var x = 0;
            foreach (var ch in line)
            {
                x += 1;
                switch (ch)
                {
                    case ' ' or '.' or '#': break;
                    default:
                        if (minDepth == 0)
                            minDepth = y;
                        maxDepth = y;
                        amphipods.Add(Amphipod.Parse(ch, (y, x)));
                        break;
                }
            }
        }

        return (amphipods, maxDepth - minDepth + 1);
    }
}
