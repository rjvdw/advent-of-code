// https://adventofcode.com/2021/day/17

using Day17;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var targetArea = TargetArea.Parse(File.ReadLines(args[0]));
Console.WriteLine($"The maximal height that can be reached is {targetArea.FindMaxHeight()}.");
Console.WriteLine($"There are {targetArea.FindAllValidTrajectories().Count()} possible initial velocities.");

