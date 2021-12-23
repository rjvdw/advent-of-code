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
        candidates.Enqueue(initialCandidate, amphipods.Count);
        costs[initialCandidate] = 0;

        int? cheapest = null;

        while (candidates.TryDequeue(out var candidate, out _))
        {
            if (cheapest is not null && costs[candidate] >= cheapest)
                continue;

            if (CheckMoveToBurrow(candidate, candidates, costs, ref cheapest))
                continue;

            CheckMoveToHallway(candidate, candidates, costs, ref cheapest);
        }

        return cheapest;
    }

    private static bool CheckMoveToBurrow(
        Candidate candidate,
        PriorityQueue<Candidate, int> candidates,
        IDictionary<Candidate, int> costs,
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

                var nextAmphipods = new List<(Node, Amphipod)>(candidate.Amphipods)
                    { [amphipod.Index] = (moveToBurrow, amphipod) };
                var nextExhausted = new List<bool>(candidate.Exhausted)
                    { [amphipod.Index] = true };
                var nextCandidate = new Candidate(nextAmphipods, nextExhausted, candidate.BurrowDepth).Normalized();

                if (nextCandidate.IsDone())
                {
                    if (cheapest == null || cheapest > costSoFar)
                    {
                        Console.WriteLine($"Cheapest so far: {costSoFar}");
                        cheapest = costSoFar;
                    }
                }
                else if (costs.ContainsKey(nextCandidate))
                {
                    if (costSoFar < costs[nextCandidate])
                    {
                        candidates.Enqueue(nextCandidate, nextCandidate.Exhausted.Count(v => !v));
                        costs[nextCandidate] = costSoFar;
                    }
                }
                else
                {
                    candidates.Enqueue(nextCandidate, nextCandidate.Exhausted.Count(v => !v));
                    costs[nextCandidate] = costSoFar;
                }
            }
        }

        return false;
    }

    private static void CheckMoveToHallway(
        Candidate candidate,
        PriorityQueue<Candidate, int> candidates,
        IDictionary<Candidate, int> costs,
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

            foreach (var neighbour in candidate.FindMovesToHallway(node, amphipod))
            {
                var cost = costSoFar + amphipod.ComputeEnergy(node.DistanceTo(neighbour));

                if (cost >= cheapest)
                    continue;

                var nextAmphipods = new List<(Node, Amphipod)>(candidate.Amphipods)
                    { [amphipod.Index] = (neighbour, amphipod) };
                var nextCandidate =
                    new Candidate(nextAmphipods, candidate.Exhausted, candidate.BurrowDepth).Normalized();

                if (nextCandidate.IsDone())
                {
                    Console.WriteLine($"Cheapest so far: {costSoFar}");
                    cheapest = cost;
                }
                else if (costs.ContainsKey(nextCandidate))
                {
                    if (costSoFar < costs[nextCandidate])
                    {
                        candidates.Enqueue(nextCandidate, nextCandidate.Exhausted.Count(v => !v));
                        costs[nextCandidate] = costSoFar;
                    }
                }
                else
                {
                    candidates.Enqueue(nextCandidate, nextCandidate.Exhausted.Count(v => !v));
                    costs[nextCandidate] = costSoFar;
                }
            }
        }
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
