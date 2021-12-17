using System;
using System.Linq;
using Xunit;

namespace Day17.Test;

public class TargetAreaTest
{
    [Fact]
    public void TestFindMaxHeight()
    {
        Assert.Equal(45, GetTestData("target area: x=20..30, y=-10..-5").FindMaxHeight());
        Assert.Equal(55, GetTestData("target area: x=20..30, y=5..10").FindMaxHeight());
        Assert.Throws<InvalidOperationException>(() => GetTestData("target area: x=20..30, y=-10..10").FindMaxHeight());
    }

    [Fact]
    public void FindAllValidTrajectories()
    {
        Assert.Equal(112, GetTestData("target area: x=20..30, y=-10..-5").FindAllValidTrajectories().Count());
    }

    private static TargetArea GetTestData(string spec) => TargetArea.Parse(new[] { spec });
}
