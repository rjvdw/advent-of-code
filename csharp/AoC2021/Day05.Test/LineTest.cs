using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day05.Test;

public class LineTest
{
    [Fact]
    public void TestGetPoints1()
    {
        var line = Line.Parse("1,1 -> 1,3");
        Assert.Equal(new List<Point> { new(1, 1), new(1, 2), new(1, 3) }, line.GetPoints().ToList());
    }

    [Fact]
    public void TestGetPoints2()
    {
        var line = Line.Parse("9,7 -> 7,7");
        Assert.Equal(new List<Point> { new(9, 7), new(8, 7), new(7, 7) }, line.GetPoints().ToList());
    }

    [Fact]
    public void TestGetPoints3()
    {
        var line = Line.Parse("1,1 -> 3,3");
        Assert.Equal(new List<Point> { new(1, 1), new(2, 2), new(3, 3) }, line.GetPoints().ToList());
    }

    [Fact]
    public void TestGetPoints4()
    {
        var line = Line.Parse("9,7 -> 7,9");
        Assert.Equal(new List<Point> { new(9, 7), new(8, 8), new(7, 9) }, line.GetPoints().ToList());
    }
}
