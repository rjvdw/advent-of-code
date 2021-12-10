namespace Day10;

/// https://adventofcode.com/2021/day/10
public static class Program
{
    public static void Main(string[] args)
    {
        if (args.Length != 1)
        {
            Console.Error.WriteLine("Usage: $0 <input file>");
            Environment.Exit(1);
            return;
        }

        var results = File
            .ReadLines(args[0])
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
