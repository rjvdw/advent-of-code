using System.Diagnostics.CodeAnalysis;

namespace RdclDev.AdventOfCode;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        Console.WriteLine($"Hello, World! [{string.Join(", ", input)}]");
    }
}
