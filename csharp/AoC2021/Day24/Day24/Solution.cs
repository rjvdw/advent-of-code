using System.Diagnostics.CodeAnalysis;

namespace Day24;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var subRoutines = AnalyzeInstructions(input);

        var max = FindModelNumber(subRoutines, Mode.Largest);
        if (max is not null)
            Console.WriteLine($"The largest valid model number is {max}.");
        else
            Console.Error.WriteLine("There are no valid model numbers.");

        var min = FindModelNumber(subRoutines, Mode.Smallest);
        if (min is not null)
            Console.WriteLine($"The smallest valid model number is {min}.");
        else
            Console.Error.WriteLine("There are no valid model numbers.");
    }

    private static List<SubRoutine> AnalyzeInstructions(IEnumerable<string> input)
    {
        var analysis = new List<(bool Divides, int ConstantX, int ConstantY)>();
        var position = 0;
        var idx = 0;

        foreach (var instruction in input)
        {
            if (instruction == "inp w")
            {
                analysis.Add((false, 0, 0));
                position = analysis.Count - 1;
                idx = 1;
                continue;
            }

            var item = analysis[position];

            switch (idx)
            {
                case 4:
                    item.Divides = instruction switch
                    {
                        "div z 1" => false,
                        "div z 26" => true,
                        _ => throw InvalidInstruction(idx, position, instruction),
                    };
                    break;
                case 5:
                    if (!instruction.StartsWith("add x "))
                        throw InvalidInstruction(idx, position, instruction);
                    item.ConstantX = int.Parse(instruction[6..]);
                    break;
                case 15:
                    if (!instruction.StartsWith("add y "))
                        throw InvalidInstruction(idx, position, instruction);
                    item.ConstantY = int.Parse(instruction[6..]);
                    break;
            }

            analysis[position] = item;
            idx += 1;
        }

        return analysis
            .Select(a => new SubRoutine(a.Divides, a.ConstantX, a.ConstantY))
            .ToList();
    }

    private static FormatException InvalidInstruction(int idx, int position, string instruction) =>
        new($"Unexpected instruction on line {idx} in subroutine {position}: {instruction}");

    private static long? FindModelNumber(IReadOnlyList<SubRoutine> subRoutines, Mode mode)
    {
        var stack = new Stack<(long ModelNumber, int Position, int Z)>();
        stack.Push((0, 0, 0));

        while (stack.TryPop(out var item))
        {
            var (modelNumber, position, z) = item;

            if (position >= subRoutines.Count)
            {
                if (z == 0)
                    return modelNumber;

                continue;
            }

            var sr = subRoutines[position];

            if (sr.Divides)
            {
                var x = sr.ComputeX(z);
                if (x is >= 1 and <= 9)
                {
                    var nextModelNumber = 10L * modelNumber + x;
                    var nextZ = sr.Run(x, z);
                    stack.Push((nextModelNumber, position + 1, nextZ));
                }
            }
            else
            {
                foreach (var w in mode.GetDigits())
                {
                    var nextModelNumber = 10L * modelNumber + w;
                    var nextZ = sr.Run(w, z);
                    stack.Push((nextModelNumber, position + 1, nextZ));
                }
            }
        }

        return null;
    }
}

public enum Mode
{
    Largest,
    Smallest,
}

internal static class ModeExtensions
{
    internal static IEnumerable<int> GetDigits(this Mode mode) => mode switch
    {
        Mode.Largest => new[] { 1, 2, 3, 4, 5, 6, 7, 8, 9 },
        Mode.Smallest => new[] { 9, 8, 7, 6, 5, 4, 3, 2, 1 },
        _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, "Invalid mode"),
    };
}
