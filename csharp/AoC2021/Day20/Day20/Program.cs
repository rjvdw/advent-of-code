using Day20;

if (args.Length != 2)
{
    Console.Error.WriteLine("Usage: $0 <input file> <steps>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]), int.Parse(args[1]));
