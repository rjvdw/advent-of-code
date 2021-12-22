namespace Day03;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var len = 0;
        var values = input
            .Select(line =>
            {
                len = line.Length;
                return Convert.ToUInt16(line, 2);
            })
            .ToList();

        var (gamma, epsilon) = ComputeGammaAndEpsilonRate(values, len);
        Console.WriteLine($"The gamma rate is {gamma}, and the epsilon rate is {epsilon}, " +
                          $"so the final answer is {gamma * epsilon}");

        var rating = ComputeLifeSupportRating(values, len);
        if (rating == null)
            Console.Error.WriteLine("No life support rating could be determined.");
        else
            Console.WriteLine($"The life support rating is {rating}.");
    }

    public static (ushort, ushort) ComputeGammaAndEpsilonRate(List<ushort> values, int len)
    {
        var mask = (ushort)0b1;
        var gamma = (ushort)0;
        var epsilon = (ushort)0;

        for (var i = 0; i < len; i += 1)
        {
            var (c0, c1) = CountFrequenciesAt(values, mask);
            if (c1 > c0)
                gamma |= mask;
            if (c1 < c0)
                epsilon |= mask;
            mask *= 2;
        }

        return (gamma, epsilon);
    }

    public static int? ComputeLifeSupportRating(List<ushort> values, int len)
    {
        var oxy = values;
        var co2 = values;

        var mask = (ushort)1;
        for (var i = 1; i < len; i += 1) // i starts at 1, because mask starts at 1
            mask *= 2;

        for (var i = 0; i < len; i += 1)
        {
            oxy = FilterRating(oxy, mask, (c0, c1) => c1 >= c0);
            co2 = FilterRating(co2, mask, (c0, c1) => c1 < c0);

            if (oxy.Count == 1 && co2.Count == 1)
                return oxy[0] * co2[0];

            mask /= 2;
        }

        return null;
    }

    private static (ushort, ushort) CountFrequenciesAt(List<ushort> values, ushort mask)
    {
        var frequencies = ((ushort)0, (ushort)0);

        foreach (var value in values)
        {
            if ((value & mask) == 0)
                frequencies.Item1 += 1;
            else
                frequencies.Item2 += 1;
        }

        return frequencies;
    }

    private static List<ushort> FilterRating(List<ushort> values, ushort mask, Func<int, int, bool> determineBit)
    {
        if (values.Count == 1)
            return values;

        var (c0, c1) = CountFrequenciesAt(values, mask);
        var bit = determineBit(c0, c1) ? mask : 0;

        return values
            .Where(value => (value & mask) == bit)
            .ToList();
    }
}
