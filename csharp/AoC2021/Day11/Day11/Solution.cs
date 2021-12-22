namespace Day11;

public static class Solution
{
    public static void Solve(IEnumerable<string> input, int steps)
    {
        var map = OctopusMap.Parse(input);
        var nrFlashes = RunSimulation(map, steps);
        Console.WriteLine($"After {steps} steps, there have been {nrFlashes} flashes.");
    }

    public static void Solve(IEnumerable<string> input)
    {
        var map = OctopusMap.Parse(input);
        var steps = RunSimulationUntil(map, flashes => flashes == map.Count);
        Console.WriteLine($"After {steps} steps, all octopuses flash at the same time.");
    }

    public static int RunSimulation(OctopusMap map, int steps)
    {
        var total = 0;
        var current = map;
        for (var i = 0; i < steps; i += 1)
        {
            var (next, flashes) = current.Tick();
            total += flashes;
            current = next;
        }

        return total;
    }

    public static int RunSimulationUntil(OctopusMap map, Func<int, bool> condition)
    {
        var counter = 0;
        var current = map;
        while (true)
        {
            counter += 1;
            var (next, flashes) = current.Tick();
            if (condition(flashes))
                return counter;
            current = next;
        }
    }
}
