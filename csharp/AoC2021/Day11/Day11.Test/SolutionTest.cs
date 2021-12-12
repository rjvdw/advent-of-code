using Xunit;

namespace Day11.Test;

public class SolutionTest
{
    [Fact]
    public void TestRunSimulation()
    {
        Assert.Equal(1656, Solution.RunSimulation(GetTestData(), 100));
    }

    [Fact]
    public void TestRunSimulationUntil()
    {
        Assert.Equal(195, Solution.RunSimulationUntil(GetTestData(), f => f == 100));
    }

    private static OctopusMap GetTestData() =>
        OctopusMap.Parse(new[]
        {
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        });
}
