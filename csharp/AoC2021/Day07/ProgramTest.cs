using Xunit;

namespace Day07;

public class ProgramTest
{
    [Fact]
    public void TestComputeFuelCostNaive()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal((uint)37, Program.ComputeFuelCostNaive(values, 2));
    }

    [Fact]
    public void TestComputeFuelCostCorrect()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal((uint)206, Program.ComputeFuelCostCorrect(values, 2));
        Assert.Equal((uint)168, Program.ComputeFuelCostCorrect(values, 5));
    }

    [Fact]
    public void TestFindOptimalPoint()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal(((uint)2, (uint)37), Program.FindOptimalPoint(values, Program.ComputeFuelCostNaive));
        Assert.Equal(((uint)5, (uint)168), Program.FindOptimalPoint(values, Program.ComputeFuelCostCorrect));
    }
}
