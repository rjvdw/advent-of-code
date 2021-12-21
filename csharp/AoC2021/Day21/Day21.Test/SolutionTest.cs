using Xunit;

namespace Day21.Test;

public class SolutionTest
{
    [Fact]
    public void TestPlayPart1()
    {
        var players = new[]
        {
            Player.Parse("Player 1 starting position: 4"),
            Player.Parse("Player 2 starting position: 8"),
        };

        var (winningTurn, losingPlayer, rolls) = Solution.PlayPart1(players, 1000);

        Assert.Equal(0, winningTurn);
        Assert.Equal(993, rolls);
        Assert.Equal(745, losingPlayer.Score);
    }

    [Fact]
    public void TestPlayPart2()
    {
        var players = new[]
        {
            Player.Parse("Player 1 starting position: 4"),
            Player.Parse("Player 2 starting position: 8"),
        };

        Assert.Equal(9632852745, Solution.PlayPart2(players, 14));
        Assert.Equal(444356092776315, Solution.PlayPart2(players, 21));
    }
}
