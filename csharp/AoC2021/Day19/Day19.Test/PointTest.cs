using System;
using Xunit;

namespace Day19.Test;

public class PointTest
{
    [Fact]
    public void TestRotate()
    {
        Assert.Equal(new Point(3, 2, 1), new Point(3, 2, 1).Rotate(0));
        Assert.Equal(new Point(3, 1, -2), new Point(3, 2, 1).Rotate(1));
        Assert.Equal(new Point(3, -1, 2), new Point(3, 2, 1).Rotate(2));
        Assert.Equal(new Point(3, -2, -1), new Point(3, 2, 1).Rotate(3));
        Assert.Equal(new Point(2, 3, -1), new Point(3, 2, 1).Rotate(4));
        Assert.Equal(new Point(2, 1, 3), new Point(3, 2, 1).Rotate(5));
        Assert.Equal(new Point(2, -1, -3), new Point(3, 2, 1).Rotate(6));
        Assert.Equal(new Point(2, -3, 1), new Point(3, 2, 1).Rotate(7));
        Assert.Equal(new Point(1, 3, 2), new Point(3, 2, 1).Rotate(8));
        Assert.Equal(new Point(1, 2, -3), new Point(3, 2, 1).Rotate(9));
        Assert.Equal(new Point(1, -2, 3), new Point(3, 2, 1).Rotate(10));
        Assert.Equal(new Point(1, -3, -2), new Point(3, 2, 1).Rotate(11));
        Assert.Equal(new Point(-1, 3, -2), new Point(3, 2, 1).Rotate(12));
        Assert.Equal(new Point(-1, 2, 3), new Point(3, 2, 1).Rotate(13));
        Assert.Equal(new Point(-1, -2, -3), new Point(3, 2, 1).Rotate(14));
        Assert.Equal(new Point(-1, -3, 2), new Point(3, 2, 1).Rotate(15));
        Assert.Equal(new Point(-2, 3, 1), new Point(3, 2, 1).Rotate(16));
        Assert.Equal(new Point(-2, 1, -3), new Point(3, 2, 1).Rotate(17));
        Assert.Equal(new Point(-2, -1, 3), new Point(3, 2, 1).Rotate(18));
        Assert.Equal(new Point(-2, -3, -1), new Point(3, 2, 1).Rotate(19));
        Assert.Equal(new Point(-3, 2, -1), new Point(3, 2, 1).Rotate(20));
        Assert.Equal(new Point(-3, 1, 2), new Point(3, 2, 1).Rotate(21));
        Assert.Equal(new Point(-3, -1, -2), new Point(3, 2, 1).Rotate(22));
        Assert.Equal(new Point(-3, -2, 1), new Point(3, 2, 1).Rotate(23));
    }

    [Fact]
    public void TestDistanceTo()
    {
        Assert.Equal(15 + 5 + 5, new Point(5, 5, 5).DistanceTo(new Point(-10, 10, 0)));
    }

    [Fact]
    public void TestAdd()
    {
        Assert.Equal(new Point(5, 7, 9), new Point(1, 2, 3) + new Point(4, 5, 6));
    }

    [Fact]
    public void TestSub()
    {
        Assert.Equal(new Point(-3, -3, -3), new Point(1, 2, 3) - new Point(4, 5, 6));
    }

    [Fact]
    public void TestParse()
    {
        Assert.Equal(new Point(1, 2, 3), Point.Parse("1,2,3"));
        Assert.Equal(new Point(-1, -2, -3), Point.Parse("-1,-2,-3"));
        Assert.Throws<FormatException>(() => Point.Parse("1,2"));
        Assert.Throws<FormatException>(() => Point.Parse("a,b,c"));
        Assert.Throws<FormatException>(() => Point.Parse("invalid"));
    }
}
