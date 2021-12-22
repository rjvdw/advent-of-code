using System.Diagnostics.CodeAnalysis;

namespace Day15;

public static class Solution
{
    [ExcludeFromCodeCoverage]
    public static void Solve(IEnumerable<string> input)
    {
        var riskLevels = RiskLevels.Parse(input);
        Console.Write("In the small cave: ");
        Console.Out.Flush();
        var score1 = riskLevels.FindShortestPath();
        Console.WriteLine(score1.HasValue
            ? $"The lowest possible risk score is {score1.Value}."
            : "There is no path through this cave.");

        var transformed = riskLevels.Transform();
        Console.Write("In the big cave: ");
        Console.Out.Flush();
        var score2 = transformed.FindShortestPath();
        Console.WriteLine(score2.HasValue
            ? $"The lowest possible risk score is {score2.Value}."
            : "There is no path through this cave.");
    }
}
