using System.Diagnostics.CodeAnalysis;

namespace Day01;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input, int windowSize)
    {
        var numbers = input.Select(int.Parse).ToList();
        Console.WriteLine(CountIncreases(numbers, windowSize));
    }

    public static int CountIncreases(List<int> numbers, int windowSize)
    {
        var count = 0;
        var window = numbers.Take(windowSize).ToList();
        var index = 0;
        var previousSum = window.Sum();

        foreach (var number in numbers.Skip(windowSize))
        {
            var sum = previousSum - window[index] + number;
            if (sum > previousSum)
                count += 1;
            window[index] = number;
            index = (index + 1) % windowSize;
            previousSum = sum;
        }

        return count;
    }
}
