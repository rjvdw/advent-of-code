using Day01;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <window size>");
    Environment.Exit(1);
}

var numbers = File
    .ReadAllLines(args[0])
    .Select(int.Parse)
    .ToList();
var windowSize = int.Parse(args[1]);

Console.WriteLine(Solution.CountIncreases(numbers, windowSize));
