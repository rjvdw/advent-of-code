// https://adventofcode.com/2021/day/6

using Day06;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <days>");
    Environment.Exit(1);
}

var values = File
    .ReadAllLines(args[0])
    .SelectMany(line => line.Split(','))
    .Select(ulong.Parse)
    .ToList();
var days = ulong.Parse(args[1]);

Console.WriteLine($"After {days} days, there are {Solution.Solve(values, days)} lantern fish.");
