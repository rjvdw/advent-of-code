// https://adventofcode.com/2021/day/16

using Day16;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var packet = Packet.Parse(File.ReadLines(args[0]));

Console.WriteLine($"The sum of the versions is {packet.SumVersions()}.");
Console.WriteLine($"The transmission evaluates to {packet.Eval()}.");
