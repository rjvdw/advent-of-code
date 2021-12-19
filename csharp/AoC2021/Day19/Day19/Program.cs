// https://adventofcode.com/.......

using Day19;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var lines = File.ReadLines(args[0]);

Console.WriteLine(Solution.Solve(lines));
