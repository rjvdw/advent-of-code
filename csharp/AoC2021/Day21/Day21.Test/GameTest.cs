using Xunit;

namespace Day21.Test;

public class GameTest
{
    [Fact]
    public void TestGame()
    {
        var players = new[]
        {
            Player.Parse("Player 1 starting position: 1"),
            Player.Parse("Player 2 starting position: 1"),
        };

        var game = new Game(players, 20);
        int? turn;
        (game, turn) = game.Play(9); // p1 -> 10
        Assert.Null(turn);
        (game, turn) = game.Play(9); // p2 -> 10
        Assert.Null(turn);
        (_, turn) = game.Play(10); // p1 -> 20
        Assert.NotNull(turn);
    }
}
