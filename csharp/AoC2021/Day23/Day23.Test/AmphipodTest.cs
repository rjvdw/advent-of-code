using Xunit;

namespace Day23.Test;

public class AmphipodTest
{
    [Fact]
    public void TestCost()
    {
        var a = Amphipod.Parse('A', (3, 3));
        var b = Amphipod.Parse('B', (3, 3));
        var c = Amphipod.Parse('C', (3, 3));
        var d = Amphipod.Parse('D', (3, 3));

        var to1 = new Node(1, 4);
        Assert.Equal(3, a.ComputeCost(to1));
        Assert.Equal(30, b.ComputeCost(to1));
        Assert.Equal(300, c.ComputeCost(to1));
        Assert.Equal(3000, d.ComputeCost(to1));

        var to2 = new Node(2, 5);
        Assert.Equal(5, a.ComputeCost(to2));
        Assert.Equal(50, b.ComputeCost(to2));
        Assert.Equal(500, c.ComputeCost(to2));
        Assert.Equal(5000, d.ComputeCost(to2));
    }

    [Fact]
    public void TestWithLocation()
    {
        Assert.Equal(Amphipod.Parse('A', (1, 2)), Amphipod.Parse('A', (1, 3)).WithLocation((1, 2)));
    }
}
