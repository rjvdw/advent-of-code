// https://adventofcode.com/2021/day/19

using Day19;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var scanners = Scanner.Parse(File.ReadLines(args[0])).ToList();
var corrected = Solution.CorrectScanners(scanners).ToList();
var beacons = Solution.FindBeacons(corrected);
Console.WriteLine($"There are {beacons.Count} beacons.");
Console.WriteLine($"The greatest distance between two scanners is {Solution.MaxDistance(corrected)}.");
