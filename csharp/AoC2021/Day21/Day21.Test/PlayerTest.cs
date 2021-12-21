using System;
using Xunit;

namespace Day21.Test;

public class PlayerTest
{
    [Fact]
    public void TestParse()
    {
        Assert.Equal(0, Player.Parse("Test Player starting position: 1").Score);
        Assert.Throws<FormatException>(() => Player.Parse("invalid_input_string"));
        Assert.Throws<FormatException>(() => Player.Parse("Test Player starting position: X"));
    }

    [Fact]
    public void TestMove()
    {
        var player = Player.Parse("Test Player starting position: 1");

        player = player.Move(1 + 2 + 3);
        Assert.Equal(7, player.Score);

        player = player.Move(4 + 5 + 6);
        Assert.Equal(9, player.Score);
    }

    [Fact]
    public void TestHasWon()
    {
        var player = Player.Parse("Test Player starting position: 1");

        player = player.Move(4);
        Assert.Equal(5, player.Score);

        Assert.True(player.HasWon(4));
        Assert.True(player.HasWon(5));
        Assert.False(player.HasWon(6));
    }
}
