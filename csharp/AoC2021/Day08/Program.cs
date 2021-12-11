// https://adventofcode.com/2021/day/8

using Day08;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var displays = File.ReadLines(args[0]).Select(Display.Parse).ToList();

Console.WriteLine($"There are {Solution.CountEasyDigits(displays)} easy digits in the output.");
Console.WriteLine($"The sum of all the displays is {Solution.DecodeDisplays(displays)}.");
