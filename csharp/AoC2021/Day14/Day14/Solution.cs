using System.Diagnostics.CodeAnalysis;

namespace Day14;

using InstructionMap = Dictionary<Pair, (Pair, Pair)>;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input, int steps)
    {
        var (polymer, instructions) = Parse(input);

        var counts = Process(polymer, instructions, steps);
        var min = long.MaxValue;
        var max = long.MinValue;
        foreach (var count in counts.Values)
        {
            if (count < min) min = count;
            if (count > max) max = count;
        }

        Console.WriteLine($"The most common element occurs {max} times and the least common element occurs {min} times. " +
                          $"The final answer is {max - min}.");

    }

    public static Dictionary<char, long> Process(string polymer, InstructionMap instructions, int steps)
    {
        var ch1 = polymer[0];

        var frequencies = new Dictionary<Pair, long>();
        foreach (var ch2 in polymer[1..])
        {
            var pair = new Pair(ch1, ch2);
            if (!frequencies.ContainsKey(pair))
                frequencies[pair] = 0;
            frequencies[pair] += 1;
            ch1 = ch2;
        }

        for (var step = 1; step <= steps; step += 1)
        {
            var next = new Dictionary<Pair, long>();
            foreach (var (pair, count) in frequencies)
            {
                var (p1, p2) = instructions[pair];
                if (!next.ContainsKey(p1))
                    next[p1] = 0;
                next[p1] += count;
                if (!next.ContainsKey(p2))
                    next[p2] = 0;
                next[p2] += count;
            }
            frequencies = next;
        }

        var counts = new Dictionary<char, long> { [ch1] = 1 };
        foreach (var ((c, _), count) in frequencies)
        {
            if (!counts.ContainsKey(c))
                counts[c] = 0;
            counts[c] += count;
        }

        return counts;
    }

    public static (string, InstructionMap) Parse(IEnumerable<string> lines)
    {
        using var enumerator = lines.GetEnumerator();

        // first line contains the input polymer
        if (!enumerator.MoveNext())
            throw new ArgumentException("Invalid input", nameof(lines));
        var polymer = enumerator.Current;

        // next line must be empty
        if (!enumerator.MoveNext() || !string.IsNullOrEmpty(enumerator.Current))
            throw new ArgumentException("Invalid input", nameof(lines));

        // remaining lines contain the instructions
        var instructions = new Dictionary<Pair, (Pair, Pair)>();
        while (enumerator.MoveNext())
        {
            var line = enumerator.Current;
            var from = new Pair(line[0], line[1]);
            var to1 = new Pair(line[0], line[6]);
            var to2 = new Pair(line[6], line[1]);
            instructions[from] = (to1, to2);
        }

        return (polymer, instructions);
    }
}
