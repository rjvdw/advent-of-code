// https://adventofcode.com/2021/day/15

using Day15;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var riskLevels = RiskLevels.Parse(File.ReadLines(args[0]));
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
