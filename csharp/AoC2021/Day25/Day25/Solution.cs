using System.Diagnostics.CodeAnalysis;

namespace Day25;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var seaFloor = SeaFloor.Parse(input);

        var i = 0;
        while (seaFloor.TryNext(out var next))
        {
            seaFloor = next;
            i += 1;
        }

        Console.WriteLine($"Step {i + 1} is the first step where no sea cucumbers move.");
    }
}
