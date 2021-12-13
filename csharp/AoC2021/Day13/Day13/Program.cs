// https://adventofcode.com/2021/day/13

using Day13;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var manual = Manual.Parse(File.ReadLines(args[0]));

Console.WriteLine($"Initially, there are {manual.VisibleDots} dots visible in the manual.");
var count = 0;
while (manual.FoldsRemaining > 0)
{
    manual.Fold();
    count += 1;
    Console.WriteLine($"After {count} folds, there are {manual.VisibleDots} dots visible in the manual");
}

Console.WriteLine("The manual now looks like this:");
Console.WriteLine(manual);
