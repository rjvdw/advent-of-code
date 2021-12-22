using System.Diagnostics.CodeAnalysis;

namespace Day18;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var lines = input.Select(SnailNumber.Parse).ToList();

        Console.WriteLine($"The magnitude of the sum of the inputs is {DoHomeWork1(lines)}.");
        Console.WriteLine($"The largest magnitude from any two numbers is {DoHomeWork2(lines)}.");
    }

    public static long DoHomeWork1(IEnumerable<SnailNumber> snailNumbers) =>
        snailNumbers.Aggregate((a, b) => a + b).Magnitude();

    public static long DoHomeWork2(List<SnailNumber> snailNumbers)
    {
        var max = long.MinValue;

        for (var i = 0; i < snailNumbers.Count; i += 1)
        {
            for (var j = 0; j < snailNumbers.Count; j += 1)
            {
                if (i != j)
                {
                    var m = (snailNumbers[i] + snailNumbers[j]).Magnitude();
                    if (m > max)
                        max = m;
                }
            }
        }

        return max;
    }
}
