namespace Day21;

public static class Solution
{
    private static readonly (int Roll, long Frequency)[] DiracOutcomes =
    {
        (3, 1L), // 111
        (4, 3L), // 112 121 211
        (5, 6L), // 113 131 311 122 212 221
        (6, 7L), // 123 132 213 231 312 321 222
        (7, 6L), // 223 232 322 133 313 331
        (8, 3L), // 233 323 332
        (9, 1L), // 333
    };

    public static (int WinningTurn, Player LosingPlayer, int Rolls) PlayPart1(Player[] players, int targetScore)
    {
        var game = new Game(players, targetScore);
        var die = new DeterministicDie(100);

        while (true)
        {
            var result = game.Play(die.Roll(3));
            if (result.HasValue)
            {
                var (turn, p) = result.Value;
                return (turn, p[turn ^ 1], die.Rolls);
            }
        }
    }

    public static long PlayPart2(Player[] players, int targetScore)
    {
        var tally = new[] { 0L, 0L };
        var games = new Stack<(long, Game Game)>();
        games.Push((1, new Game(players, targetScore)));

        while (games.TryPop(out var popped))
        {
            var (multiplier, game) = popped;
            foreach (var (roll, frequency) in DiracOutcomes)
            {
                var g = game.Duplicate();
                var freq = multiplier * frequency;
                var result = g.Play(roll);
                if (result.HasValue)
                    tally[result.Value.Turn] += freq;
                else
                    games.Push((freq, g));
            }
        }

        return tally[0] > tally[1] ? tally[0] : tally[1];
    }
}
