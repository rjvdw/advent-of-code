using Day21;

if (args.Length != 3)
{
    Console.Error.WriteLine("Usage: $0 <input file> <part1|part2> <target score>");
    Environment.Exit(1);
}

var players = File.ReadLines(args[0])
    .Take(2) // For simplicity, assume only two players.
    .Select(Player.Parse)
    .ToArray();
var part = args[1];
var targetScore = int.Parse(args[2]);

if (part == "part1")
{
    var (winningTurn, loser, rolls) = Solution.PlayPart1(players, targetScore);
    Console.WriteLine($"Player #{2 - winningTurn} loses after {rolls} rolls, " +
                      $"making the final answer {loser.Score * rolls}.");
}
else
{
    Console.WriteLine($"The winning player wins in {Solution.PlayPart2(players, targetScore)} universes.");
}
