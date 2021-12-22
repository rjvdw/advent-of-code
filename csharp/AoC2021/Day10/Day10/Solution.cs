namespace Day10;

public static class Solution
{
    public static void Solve(IEnumerable<string> input)
    {
        var results = input
            .Select(Analysis.Parse)
            .ToList();

        var invalidScore = results
            .Where(analysis => analysis.Type == Analysis.AnalysisType.Invalid)
            .Select(analysis => analysis.Score())
            .Aggregate((acc, v) => acc + v);

        Console.WriteLine($"The total score of all invalid lines is {invalidScore}.");

        var validScores = results
            .Where(analysis => analysis.Type == Analysis.AnalysisType.Valid)
            .Select(analysis => analysis.Score())
            .ToList();
        validScores.Sort();

        Console.WriteLine($"The middle score of all valid lines is {validScores[validScores.Count / 2]}.");
    }
}
