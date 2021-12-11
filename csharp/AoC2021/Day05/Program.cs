// https://adventofcode.com/2021/day/5

using Day05;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var lines = File.ReadAllLines(args[0]).Select(Line.Parse).ToList();

var count1 = Solution.CountDangerousPoints(lines, false);
Console.WriteLine($"Not considering diagonals, there are {count1} points where multiple lines overlap.");

var count2 = Solution.CountDangerousPoints(lines, true);
Console.WriteLine($"Considering diagonals, there are {count2} points where multiple lines overlap.");
