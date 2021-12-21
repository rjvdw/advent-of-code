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
            var (nextGame, turn) = game.Play(die.Roll(3));
            game = nextGame;
            if (turn.HasValue)
            {
                return (turn.Value, game.Players[turn.Value ^ 1], die.Rolls);
            }
        }
    }

    public static long PlayPart2(Player[] players, int targetScore)
    {
        var game = new Game(players, targetScore);
        var cache = new Dictionary<Game, long[]>();

        return PlayPart2Recursive(game, cache).Max();
    }

    private static long[] PlayPart2Recursive(Game game, IDictionary<Game, long[]> tallies)
    {
        if (tallies.ContainsKey(game))
            return tallies[game];

        var tally = new[] { 0L, 0L };
        foreach (var (roll, frequency) in DiracOutcomes)
        {
            var (nextGame, winningTurn) = game.Play(roll);
            if (winningTurn.HasValue)
            {
                tally[winningTurn.Value] += frequency;
            }
            else
            {
                var t = PlayPart2Recursive(nextGame, tallies);
                tally[0] += frequency * t[0];
                tally[1] += frequency * t[1];
            }
        }

        tallies[game] = tally;
        return tally;
    }
}
