using System;
using System.Collections.Generic;
using Xunit;

namespace Day22.Test;

public class RangeTest
{
    [Fact]
    public void TestFitsWithin()
    {
        var a = new Range(-5, 5);
        var b = new Range(-10, 10);
        var c = new Range(0, 10);

        Assert.True(a.FitsWithin(a));
        Assert.True(a.FitsWithin(b));
        Assert.False(a.FitsWithin(c));
        Assert.False(b.FitsWithin(a));
        Assert.True(b.FitsWithin(b));
        Assert.False(b.FitsWithin(c));
        Assert.False(c.FitsWithin(a));
        Assert.True(c.FitsWithin(b));
        Assert.True(c.FitsWithin(c));
    }

    [Fact]
    public void TestSize()
    {
        Assert.Equal(20, new Range(-7, 12).Size());
        Assert.Equal(6, new Range(7, 12).Size());
    }

    [Fact]
    public void TestPartition()
    {
        var range = new Range(0, 10);

        var expected1 = new List<Range>
        {
            new(5, 10),
            new(0, 4),
        };
        Assert.Equal(expected1, range.Partition(new Range(5, 15)));

        var expected2 = new List<Range> { range };
        Assert.Equal(expected2, range.Partition(new Range(15, 25)));

        var expected3 = new List<Range>
        {
            new(2, 7),
            new(0, 1),
            new(8, 10),
        };
        Assert.Equal(expected3, range.Partition(new Range(2, 7)));

        var expected4 = new List<Range> { range };
        Assert.Equal(expected4, range.Partition(new Range(-5, 15)));
    }

    [Fact]
    public void TestParse()
    {
        Assert.Equal(new Range(-5, 5), Range.Parse("-5..5"));
        Assert.Throws<FormatException>(() => Range.Parse("invalid"));
        Assert.Throws<FormatException>(() => Range.Parse("x..x"));
        Assert.Throws<FormatException>(() => Range.Parse("5..-5"));
    }
}
