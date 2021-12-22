using Day10;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]));
