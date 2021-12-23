using System.Diagnostics.CodeAnalysis;

namespace Day23;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var (amphipods, burrowDepth) = Parse(input);
        var cheapestPath = FindCheapestPath(amphipods, burrowDepth);
        if (cheapestPath.HasValue)
            Console.WriteLine($"The cheapest solution has cost {cheapestPath}");
        else
            Console.Error.WriteLine("No solution exists.");
    }

    public static int? FindCheapestPath(List<(Node, Amphipod)> amphipods, int burrowDepth)
    {
        var candidates = new PriorityQueue<Candidate, int>();
        var costs = new Dictionary<Candidate, int>();

        var initialCandidate = new Candidate(
            amphipods,
            Enumerable.Repeat(false, amphipods.Count).ToList(),
            burrowDepth
        ).Normalized();
        candidates.Enqueue(initialCandidate, 0);
        costs[initialCandidate] = 0;

        int? cheapest = null;

        while (candidates.TryDequeue(out var candidate, out _))
        {
            if (cheapest is not null && costs[candidate] >= cheapest)
                continue;

            if (CheckMoveToBurrow(candidate, candidates, costs, ref cheapest))
            {
                continue;
            }

            CheckMoveToHallway(candidate, candidates, costs, ref cheapest);
        }

        return cheapest;
    }

    private static bool CheckMoveToBurrow(
        Candidate candidate,
        PriorityQueue<Candidate, int> candidates,
        Dictionary<Candidate, int> costs,
        ref int? cheapest
    )
    {
        var costSoFar = costs[candidate];

        foreach (var (node, amphipod) in candidate.Amphipods)
        {
            if (candidate.Exhausted[amphipod.Index])
                continue;

            if (candidate.ExitIsBlocked(node))
                continue;

            var moveToBurrow = candidate.FindMoveToBurrow(node, amphipod);
            if (moveToBurrow is not null)
            {
                var cost = costSoFar + amphipod.ComputeEnergy(node.DistanceTo(moveToBurrow));

                if (cheapest is not null && cost >= cheapest)
                    return true;

                //
            }
        }

        return false;
    }

    private static void CheckMoveToHallway(
        Candidate candidate,
        PriorityQueue<Candidate, int> candidates,
        Dictionary<Candidate, int> costs,
        ref int? cheapest
    )
    {
        //
    }

    public static (List<(Node, Amphipod)> Amphipods, int BurrowDepth) Parse(IEnumerable<string> input)
    {
        var amphipods = new List<(Node, Amphipod)>();
        var burrowMin = 0;
        var burrowMax = 0;

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
                    case ' ' or '.' or '#':
                        break;
                    default:
                        if (burrowMin == 0)
                            burrowMin = y;
                        burrowMax = y;
                        amphipods.Add((new Node(y, x), Amphipod.Parse(ch, amphipods.Count)));
                        break;
                }
            }
        }

        return (amphipods, burrowMax - burrowMin + 1);
    }
}
