using System.Diagnostics.CodeAnalysis;

namespace Day13;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var manual = Manual.Parse(input);

        Console.WriteLine($"Initially, there are {manual.VisibleDots} dots visible in the manual.");
        var count = 0;
        while (manual.FoldsRemaining > 0)
        {
            manual.Fold();
            count += 1;
            Console.WriteLine($"After {count} folds, there are {manual.VisibleDots} dots visible in the manual");
        }

        Console.WriteLine("The manual now looks like this:");
        Console.WriteLine(manual);
    }
}
