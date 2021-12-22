using System;
using Xunit;

namespace Day22.Test;

public class CuboidTest
{
    [Fact]
    public void TestFitsWithin()
    {
        var a = new Cuboid(
            true,
            new Range(-5, 5),
            new Range(-5, 5),
            new Range(-5, 5)
        );

        var b = new Cuboid(
            true,
            new Range(-10, 10),
            new Range(-10, 10),
            new Range(-10, 10)
        );

        Assert.True(a.FitsWithin(a));
        Assert.True(a.FitsWithin(b));
        Assert.False(b.FitsWithin(a));
        Assert.True(b.FitsWithin(b));
    }

    [Fact]
    public void TestSize()
    {
        var cuboid = new Cuboid(
            true,
            new Range(-5, 5),
            new Range(-5, 5),
            new Range(-5, 5)
        );

        Assert.Equal(11 * 11 * 11, cuboid.Size());
    }

    [Fact]
    public void TestParse1()
    {
        var cuboid = Cuboid.Parse("on x=1..2,y=3..4,z=5..6");
        var expected = new Cuboid(
            true,
            new Range(1, 2),
            new Range(3, 4),
            new Range(5, 6)
        );
        Assert.Equal(expected, cuboid);
    }

    [Fact]
    public void TestParse2()
    {
        var cuboid = Cuboid.Parse("off x=1..2,y=3..4,z=5..6");
        var expected = new Cuboid(
            false,
            new Range(1, 2),
            new Range(3, 4),
            new Range(5, 6)
        );
        Assert.Equal(expected, cuboid);
    }

    [Fact]
    public void TestParse3()
    {
        Assert.Throws<FormatException>(() => Cuboid.Parse("invalid"));
        Assert.Throws<FormatException>(() => Cuboid.Parse("on x=1..2,invalid"));
        Assert.Throws<FormatException>(() => Cuboid.Parse("on x=1..2,y=3..4,invalid"));
    }
}
