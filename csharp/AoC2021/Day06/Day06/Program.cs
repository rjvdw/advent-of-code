using Day06;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <days>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]), ulong.Parse(args[1]));
