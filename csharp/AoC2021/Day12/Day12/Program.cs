// https://adventofcode.com/2021/day/12

using Day12;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <max revisits>");
    Environment.Exit(1);
}

var map = CaveMap.Parse(File.ReadLines(args[0]));
var maxRevisits = int.Parse(args[1]);

Console.WriteLine($"There are {map.CountPaths(maxRevisits)} paths " +
                  $"that don't revisit a small cave more than {maxRevisits} times.");
