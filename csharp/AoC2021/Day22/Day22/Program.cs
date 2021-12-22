using Day22;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var cubes = File.ReadLines(args[0]).Select(Cuboid.Parse);

var (init, full) = Solution.Solve(cubes);

Console.WriteLine($"After the initialization sequence, {init} cubes are on.");
Console.WriteLine($"After the full reboot sequence, {full} cubes are on.");
