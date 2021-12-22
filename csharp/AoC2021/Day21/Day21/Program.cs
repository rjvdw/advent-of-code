using Day21;

if (args.Length != 3)
{
    Console.Error.WriteLine("Usage: $0 <input file> <part1|part2> <target score>");
    Environment.Exit(1);
}

Solution.Solve(File.ReadLines(args[0]), args[1], int.Parse(args[2]));
