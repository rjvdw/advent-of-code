using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day20.Test;

public class BoundsTest
{
    [Fact]
    public void TestUpdateWith()
    {
        var bounds = new Bounds((long.MaxValue, long.MaxValue), (long.MinValue, long.MinValue));
        bounds.UpdateWith(-5, 5);
        bounds.UpdateWith(5, -5);
        Assert.Equal((-5, -5), bounds.TopLeft);
        Assert.Equal((5, 5), bounds.BottomRight);
    }

    [Fact]
    public void TestStretched()
    {
        var bounds = new Bounds((-3, -3), (3, 3)).Stretched(2);
        Assert.Equal((-5, -5), bounds.TopLeft);
        Assert.Equal((5, 5), bounds.BottomRight);
    }

    [Fact]
    public void TestIterRowCol()
    {
        var bounds = new Bounds((-1, -1), (1, 1));
        Assert.Equal(
            new List<(long, long)>
            {
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            },
            bounds.IterRowCol().ToList()
        );
    }

    [Fact]
    public void TestContains()
    {
        var bounds = new Bounds((-5, -5), (5, 5));

        Assert.True(bounds.Contains(0, 0));
        Assert.True(bounds.Contains(5, 5));
        Assert.True(bounds.Contains(5, -5));
        Assert.True(bounds.Contains(-5, 5));
        Assert.True(bounds.Contains(-5, -5));

        Assert.False(bounds.Contains(-6, -5));
        Assert.False(bounds.Contains(-5, -6));
        Assert.False(bounds.Contains(6, 5));
        Assert.False(bounds.Contains(5, 6));
    }
}
