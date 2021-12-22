using System.Diagnostics.CodeAnalysis;

namespace Day06;

public static class Solution
{
    private const ulong GestationPeriod = 9;
    private const ulong ReproductionRate = 7;

    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input, ulong days)
    {
        var values = input
            .SelectMany(line => line.Split(','))
            .Select(ulong.Parse)
            .ToList();

        Console.WriteLine($"After {days} days, there are {CountLanternFish(values, days)} lantern fish.");
    }

    public static ulong CountLanternFish(IEnumerable<ulong> values, ulong n)
    {
        var cache = new Dictionary<ulong, ulong>();

        return values
            .Select(v => CountRecursive(n + GestationPeriod - v - 1, cache))
            .Aggregate(0UL, (sum, next) => sum + next);
    }

    private static ulong CountRecursive(ulong n, IDictionary<ulong, ulong> cache)
    {
        if (cache.ContainsKey(n))
            return cache[n];

        var v = n < GestationPeriod
            ? 1
            : CountRecursive(n - GestationPeriod, cache) + CountRecursive(n - ReproductionRate, cache);
        cache[n] = v;
        return v;
    }
}
