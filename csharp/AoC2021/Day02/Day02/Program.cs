// https://adventofcode.com/2021/day/2

using Day02;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: $0 <input file>");
    Environment.Exit(1);
}

var instructions = File
    .ReadAllLines(args[0])
    .Select(Instruction.Parse)
    .ToList();

var (depth1, position1) = Solution.FollowInstructions(instructions);
Console.WriteLine($"Not considering aim, the submarine ends at position {position1} and depth {depth1}, " +
                  $"for a final answer of {position1 * depth1}.");

var (depth2, position2) = Solution.FollowInstructionsWithAim(instructions);
Console.WriteLine($"Considering aim, the submarine ends at position {position2} and depth {depth2}, " +
                  $"for a final answer of {position2 * depth2}.");
