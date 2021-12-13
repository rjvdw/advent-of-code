using System.Collections.Generic;
using Xunit;

namespace Day13.Test;

public class ManualTest
{
    [Fact]
    public void TestFold()
    {
        var manual = GetTestData();
        Assert.Equal(18, manual.VisibleDots);

        manual.Fold();
        Assert.Equal(17, manual.VisibleDots);
    }

    [Fact]
    public void TestToString()
    {
        var manual = GetTestData();
        while (manual.FoldsRemaining > 0)
            manual.Fold();

        var expected = string.Join('\n',
            "#####",
            "#...#",
            "#...#",
            "#...#",
            "#####");

        Assert.Equal(expected, manual.ToString());
    }

    private static Manual GetTestData() =>
        Manual.Parse(new List<string>
        {
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5"
        });
}
