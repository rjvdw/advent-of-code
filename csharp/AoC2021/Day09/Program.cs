namespace Day09;

/// https://adventofcode.com/2021/day/9
public static class Program
{
    public static void Main(string[] args)
    {
        if (args.Length != 1)
        {
            Console.Error.WriteLine("Usage: $0 <input file>");
            Environment.Exit(1);
            return;
        }

        var heightMap = HeightMap.Parse(File.ReadLines(args[0]));
        var lowPoints = heightMap.FindLowPoints().ToList();
        var values = lowPoints.Select(p => heightMap[p]);
        Console.WriteLine($"The sum of the risk levels is {CalculateRisk(values)}.");

        var basins = heightMap.FindBasins(lowPoints);
        var threeLargest = basins.OrderByDescending(s => s).Take(3).ToList();
        Console.WriteLine($"The three largest basins have sizes [{string.Join(", ", threeLargest)}]. " +
                          $"The final answer is {threeLargest.Aggregate((a, v) => a * v)}.");
    }

    public static uint CalculateRisk(IEnumerable<byte> values) =>
        values.Select(v => 1u + v).Aggregate((a, v) => a + v);
}
