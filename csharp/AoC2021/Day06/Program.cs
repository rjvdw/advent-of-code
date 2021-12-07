namespace Day06;

// https://adventofcode.com/2021/day/6
public static class Program
{
    private const ulong GestationPeriod = 9;
    private const ulong ReproductionRate = 7;

    public static void Main(string[] args)
    {
        if (args.Length != 2)
        {
            Console.Error.WriteLine("Usage: $0 <input file> <days>");
            Environment.Exit(1);
            return;
        }

        var values = File
            .ReadAllLines(args[0])
            .SelectMany(line => line.Split(','))
            .Select(ulong.Parse)
            .ToList();
        var days = ulong.Parse(args[1]);

        Console.WriteLine($"After {days} days, there are {Solve(values, days)} lantern fish.");
    }

    public static ulong Solve(IEnumerable<ulong> values, ulong n)
    {
        var cache = new Dictionary<ulong, ulong>();

        return values
            .Select(v => FiboAlt(n + GestationPeriod - v - 1, cache))
            .Aggregate((ulong)0, (sum, next) => sum + next);
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
