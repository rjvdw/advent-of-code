namespace Day11;

/// https://adventofcode.com/2021/day/11
public static class Program
{
    private const int MaxSteps = 10000;

    public static void Main(string[] args)
    {
        if (args.Length == 0)
        {
            Console.Error.WriteLine("Usage: $0 <input file> <steps?>");
            Environment.Exit(1);
            return;
        }

        var map = OctopusMap.Parse(File.ReadLines(args[0]));

        if (args.Length == 2)
        {
            var steps = int.Parse(args[1]);
            var nrFlashes = RunSimulation(map, steps);
            Console.WriteLine($"After {steps} steps, there have been {nrFlashes} flashes.");
        }
        else
        {
            var steps = RunSimulationUntil(map, flashes => flashes == map.Count);
            Console.WriteLine($"After {steps} steps, all octopuses flash at the same time.");
        }
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

            if (counter > MaxSteps)
            {
                Console.Error.WriteLine(
                    $"The simulation has been running for {counter} steps. This does not seem good.");
                Environment.Exit(1);
            }
        }
    }
}
