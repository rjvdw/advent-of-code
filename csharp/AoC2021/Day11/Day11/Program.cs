// https://adventofcode.com/2021/day/11

using Day11;

if (args.Length == 0)
{
    Console.Error.WriteLine("Usage: $0 <input file> <steps?>");
    Environment.Exit(1);
}

var map = OctopusMap.Parse(File.ReadLines(args[0]));

if (args.Length == 2)
{
    var steps = int.Parse(args[1]);
    var nrFlashes = Solution.RunSimulation(map, steps);
    Console.WriteLine($"After {steps} steps, there have been {nrFlashes} flashes.");
}
else
{
    var steps = Solution.RunSimulationUntil(map, flashes => flashes == map.Count);
    Console.WriteLine($"After {steps} steps, all octopuses flash at the same time.");
}
