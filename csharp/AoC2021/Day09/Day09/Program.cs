using Day09;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var heightMap = HeightMap.Parse(File.ReadLines(args[0]));
var lowPoints = heightMap.FindLowPoints().ToList();
var values = lowPoints.Select(p => heightMap[p]);
Console.WriteLine($"The sum of the risk levels is {Solution.CalculateRisk(values)}.");

var basins = heightMap.FindBasins(lowPoints);
var threeLargest = basins.OrderByDescending(s => s).Take(3).ToList();
Console.WriteLine($"The three largest basins have sizes [{string.Join(", ", threeLargest)}]. " +
                  $"The final answer is {threeLargest.Aggregate((a, v) => a * v)}.");
