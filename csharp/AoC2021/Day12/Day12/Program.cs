using Day12;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <max revisits>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]), int.Parse(args[1]));
