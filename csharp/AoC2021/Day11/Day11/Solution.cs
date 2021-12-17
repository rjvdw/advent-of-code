namespace Day11;

public static class Solution
{
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
