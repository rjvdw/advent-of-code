using Xunit;

namespace Day21.Test;

public class DeterministicDieTest
{
    [Fact]
    public void TestRoll()
    {
        var die = new DeterministicDie(6);
        Assert.Equal(1, die.Roll());
        Assert.Equal(2, die.Roll());
        Assert.Equal(3 + 4 + 5, die.Roll(3));
        Assert.Equal(6, die.Roll());
        Assert.Equal(1, die.Roll());
        Assert.Equal(2 + 3 + 4 + 5 + 6 + 1, die.Roll(6));
        Assert.Equal(13, die.Rolls);
    }
}
