namespace Day06;

public static class Solution
{
    private const ulong GestationPeriod = 9;
    private const ulong ReproductionRate = 7;

    public static void Solve(IEnumerable<string> input, ulong days)
    {
        var values = input
            .SelectMany(line => line.Split(','))
            .Select(ulong.Parse)
            .ToList();

        Console.WriteLine($"After {days} days, there are {Solve(values, days)} lantern fish.");
    }

    public static ulong Solve(IEnumerable<ulong> values, ulong n)
    {
        var cache = new Dictionary<ulong, ulong>();

        return values
            .Select(v => FiboAlt(n + GestationPeriod - v - 1, cache))
            .Aggregate(0UL, (sum, next) => sum + next);
    }

    private static ulong FiboAlt(ulong n, IDictionary<ulong, ulong> cache)
    {
        if (cache.ContainsKey(n))
            return cache[n];

        var v = n < GestationPeriod
            ? 1
            : FiboAlt(n - GestationPeriod, cache) + FiboAlt(n - ReproductionRate, cache);
        cache[n] = v;
        return v;
    }
}
