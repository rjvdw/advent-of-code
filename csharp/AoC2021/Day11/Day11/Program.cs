using Day11;

if (args.Length == 0)
{
    Console.Error.WriteLine("Usage: $0 <input file> <steps?>");
    Environment.Exit(1);
}

if (args.Length == 2)
    Solution.Solve(File.ReadLines(args[0]), int.Parse(args[1]));
else
    Solution.Solve(File.ReadLines(args[0]));
