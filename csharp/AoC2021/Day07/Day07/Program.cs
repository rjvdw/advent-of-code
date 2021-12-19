using Day07;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var points = File
    .ReadAllLines(args[0])
    .SelectMany(line => line.Split(','))
    .Select(uint.Parse)
    .ToList();

var (optimalPoint1, fuelCost1) = Solution.FindOptimalPoint(points, Solution.ComputeFuelCostNaive)!.Value;
Console.WriteLine("Using the naive fuel computation, " +
                  $"the optimal point is {optimalPoint1}, " +
                  $"with a cost of {fuelCost1}.");

var (optimalPoint2, fuelCost2) = Solution.FindOptimalPoint(points, Solution.ComputeFuelCostCorrect)!.Value;
Console.WriteLine("Using the correct fuel computation, " +
                  $"the optimal point is {optimalPoint2}, " +
                  $"with a cost of {fuelCost2}.");
