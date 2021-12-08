using Xunit;

namespace Day07;

public class ProgramTest
{
    [Fact]
    public void TestComputeFuelCostNaive()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal(37U, Program.ComputeFuelCostNaive(values, 2));
    }

    [Fact]
    public void TestComputeFuelCostCorrect()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal(206U, Program.ComputeFuelCostCorrect(values, 2));
        Assert.Equal(168U, Program.ComputeFuelCostCorrect(values, 5));
    }

    [Fact]
    public void TestFindOptimalPoint()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal((2U, 37U), Program.FindOptimalPoint(values, Program.ComputeFuelCostNaive));
        Assert.Equal((5U, 168U), Program.FindOptimalPoint(values, Program.ComputeFuelCostCorrect));
    }
}
