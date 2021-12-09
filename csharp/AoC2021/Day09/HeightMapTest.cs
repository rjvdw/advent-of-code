using Xunit;

namespace Day09;

public class HeightMapTest
{
    [Fact]
    public void TestFindLowPoints()
    {
        var heightMap = HeightMap.Parse(new[]
        {
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        });
        (int, int)[] expected = { (0, 1), (0, 9), (2, 2), (4, 6) };
        Assert.Equal(expected, heightMap.FindLowPoints());
    }

    [Fact]
    public void TestFindBasins()
    {
        var heightMap = HeightMap.Parse(new[]
        {
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        });
        int[] expected = { 3, 9, 14, 9 };
        Assert.Equal(expected, heightMap.FindBasins(heightMap.FindLowPoints()));
    }
}
