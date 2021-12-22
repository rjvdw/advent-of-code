namespace Day19;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var scanners = Scanner.Parse(input).ToList();
        var corrected = CorrectScanners(scanners).ToList();
        var beacons = FindBeacons(corrected);
        Console.WriteLine($"There are {beacons.Count} beacons.");
        Console.WriteLine($"The greatest distance between two scanners is {MaxDistance(corrected)}.");
    }

    public static IEnumerable<Scanner> CorrectScanners(List<Scanner> scanners)
    {
        var i = 0;
        var handled = new HashSet<int> { i };
        var corrected = new List<Scanner> { scanners[0] };
        yield return scanners[0];

        while (corrected.Count < scanners.Count)
        {
            foreach (var scanner in scanners)
            {
                if (handled.Contains(scanner.Idx))
                    continue;
                var c = corrected[i].Adjust(scanner);
                if (c != null)
                {
                    handled.Add(c.Idx);
                    corrected.Add(c);
                    yield return c;
                }
            }

            i += 1;
        }
    }

    public static HashSet<Point> FindBeacons(IEnumerable<Scanner> scanners) =>
        scanners
            .SelectMany(scanner => scanner.Beacons)
            .ToHashSet();

    public static long MaxDistance(List<Scanner> scanners) =>
        scanners.Aggregate(
            long.MinValue,
            (current, s1) => scanners
                .Select(s2 => s1.Position.DistanceTo(s2.Position))
                .Prepend(current)
                .Max());
}
