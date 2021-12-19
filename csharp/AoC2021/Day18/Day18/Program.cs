using Day18;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var lines = File.ReadLines(args[0]).Select(SnailNumber.Parse).ToList();

Console.WriteLine($"The magnitude of the sum of the inputs is {Solution.DoHomeWork1(lines)}.");
Console.WriteLine($"The largest magnitude from any two numbers is {Solution.DoHomeWork2(lines)}.");
