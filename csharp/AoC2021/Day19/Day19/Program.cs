// https://adventofcode.com/.......

using Day19;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var scanners = Scanner.Parse(File.ReadLines(args[0])).ToList();

foreach (var scanner in scanners)
    Console.WriteLine(scanner);
