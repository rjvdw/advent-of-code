using Xunit;

namespace Day23.Test;

public class AmphipodTest
{
    [Fact]
    public void TestParse()
    {
        Assert.Equal(new Amphipod(Color.Amber, 10), Amphipod.Parse('A', 10));
        Assert.Equal(new Amphipod(Color.Bronze, 10), Amphipod.Parse('B', 10));
        Assert.Equal(new Amphipod(Color.Copper, 10), Amphipod.Parse('C', 10));
        Assert.Equal(new Amphipod(Color.Desert, 10), Amphipod.Parse('D', 10));
    }

    [Fact]
    public void TestComputeEnergy()
    {
        var a = new Amphipod(Color.Amber, 0);
        var b = new Amphipod(Color.Bronze, 0);
        var c = new Amphipod(Color.Copper, 0);
        var d = new Amphipod(Color.Desert, 0);

        Assert.Equal(10, a.ComputeEnergy(10));
        Assert.Equal(100, b.ComputeEnergy(10));
        Assert.Equal(1000, c.ComputeEnergy(10));
        Assert.Equal(10000, d.ComputeEnergy(10));
    }

    [Fact]
    public void TestTargetBurrow()
    {
        var a = new Amphipod(Color.Amber, 0);
        var b = new Amphipod(Color.Bronze, 0);
        var c = new Amphipod(Color.Copper, 0);
        var d = new Amphipod(Color.Desert, 0);

        Assert.Equal(3, a.TargetBurrow);
        Assert.Equal(5, b.TargetBurrow);
        Assert.Equal(7, c.TargetBurrow);
        Assert.Equal(9, d.TargetBurrow);
    }

    [Fact]
    public void TestWithIndex()
    {
        Assert.Equal(10, new Amphipod(Color.Amber, 5).WithIndex(10).Index);
    }
}
