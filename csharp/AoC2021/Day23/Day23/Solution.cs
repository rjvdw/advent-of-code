using System.Diagnostics.CodeAnalysis;

namespace Day23;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var (amphipods, sideRoomDepth) = Parse(input);
        var cheapestPath = FindCheapestPath(amphipods, sideRoomDepth);
        if (cheapestPath.HasValue)
            Console.WriteLine($"The cheapest solution has cost {cheapestPath}");
        else
            Console.Error.WriteLine("No solution exists.");
    }

    public static int? FindCheapestPath(IEnumerable<Amphipod> amphipods, int sideRoomDepth)
    {
        throw new NotImplementedException("TODO");
    }

    public static (IEnumerable<Amphipod> Amphipods, int SideRoomDepth) Parse(IEnumerable<string> input)
    {
        throw new NotImplementedException("TODO");
    }
}
