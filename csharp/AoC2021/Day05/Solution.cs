namespace Day05;

public static class Solution
{
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
