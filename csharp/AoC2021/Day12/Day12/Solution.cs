namespace Day12;

public static class Solution
{
    public static void Solve(IEnumerable<string> input, int maxRevisits)
    {
        var map = CaveMap.Parse(input);

        Console.WriteLine($"There are {map.CountPaths(maxRevisits)} paths " +
                          $"that don't revisit a small cave more than {maxRevisits} times.");
    }
}
