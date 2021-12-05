namespace Day05;

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

        var inputFile = args[0];
        var lines = File.ReadAllLines(inputFile).Select(Line.Parse).ToList();

        var count1 = CountDangerousPoints(lines, false);
        Console.WriteLine($"Not considering diagonals, there are {count1} points where multiple lines overlap.");

        var count2 = CountDangerousPoints(lines, true);
        Console.WriteLine($"Considering diagonals, there are {count2} points where multiple lines overlap.");
    }

    public static int CountDangerousPoints(IEnumerable<Line> lines, bool includeDiagonals)
    {
        Dictionary<Point, int> counts = new();
        lines
            .Where(line => includeDiagonals || line.IsHorizontal() || line.IsVertical())
            .SelectMany(line => line.GetPoints())
            .ToList()
            .ForEach(point => counts[point] = counts.ContainsKey(point) ? counts[point] + 1 : 1);
        return counts.Values.Count(c => c > 1);
    }
}
