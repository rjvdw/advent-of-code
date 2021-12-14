// https://adventofcode.com/2021/day/14

using Day14;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <steps>");
    Environment.Exit(1);
}

var lines = File.ReadLines(args[0]);
var (polymer, instructions) = Solution.Parse(lines);
var steps = int.Parse(args[1]);

var counts = Solution.Process(polymer, instructions, steps);
var min = long.MaxValue;
var max = long.MinValue;
foreach (var count in counts.Values)
{
    if (count < min) min = count;
    if (count > max) max = count;
}

Console.WriteLine($"The most common element occurs {max} times and the least common element occurs {min} times. " +
                  $"The final answer is {max - min}.");
