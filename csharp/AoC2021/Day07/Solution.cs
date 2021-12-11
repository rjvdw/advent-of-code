namespace Day07;

public static class Solution
{
    public static (uint, uint)? FindOptimalPoint(List<uint> points,
        Func<IEnumerable<uint>, uint, uint> computeFuelCost)
    {
        var min = uint.MaxValue;
        var max = uint.MinValue;
        foreach (var point in points)
        {
            if (point < min) min = point;
            if (point > max) max = point;
        }

        (uint, uint)? optimum = null;
        for (var point = min; point <= max; point += 1)
        {
            var fuelCost = computeFuelCost(points, point);
            optimum = optimum switch
            {
                null => (point, fuelCost),
                var (_, c) when c > fuelCost => (point, fuelCost),
                _ => optimum
            };
        }

        return optimum;
    }

    public static uint ComputeFuelCostNaive(IEnumerable<uint> points, uint point) =>
        points
            .Select(p => Diff(p, point))
            .Aggregate((acc, p) => acc + p);

    public static uint ComputeFuelCostCorrect(IEnumerable<uint> points, uint point) =>
        points
            .Select(p => Diff(p, point))
            .Select(d => d * (d + 1) / 2)
            .Aggregate((acc, p) => acc + p);

    private static uint Diff(uint p1, uint p2) => p1 > p2 ? p1 - p2 : p2 - p1;
}
