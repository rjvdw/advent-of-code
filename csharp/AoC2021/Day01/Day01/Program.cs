using Day01;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <window size>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]), int.Parse(args[1]));
