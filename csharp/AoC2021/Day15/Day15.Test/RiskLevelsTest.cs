using Xunit;

namespace Day15.Test;

public class RiskLevelsTest
{
    [Fact]
    public void TestShortestPath1()
    {
        Assert.Equal(40, GetTestData().FindShortestPath());
    }

    [Fact]
    public void TestShortestPath2()
    {
        Assert.Equal(315, GetTestData().Transform().FindShortestPath());
    }

    private static RiskLevels GetTestData() => RiskLevels.Parse(new[]
    {
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    });
}
