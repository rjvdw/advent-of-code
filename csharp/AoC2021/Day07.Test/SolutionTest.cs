using System.Collections.Generic;
using Xunit;

namespace Day07.Test;

public class SolutionTest
{
    [Fact]
    public void TestComputeFuelCostNaive()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal(37U, Solution.ComputeFuelCostNaive(values, 2));
    }

    [Fact]
    public void TestComputeFuelCostCorrect()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal(206U, Solution.ComputeFuelCostCorrect(values, 2));
        Assert.Equal(168U, Solution.ComputeFuelCostCorrect(values, 5));
    }

    [Fact]
    public void TestFindOptimalPoint()
    {
        var values = new List<uint> { 16, 1, 2, 0, 4, 2, 7, 1, 2, 14 };
        Assert.Equal((2U, 37U), Solution.FindOptimalPoint(values, Solution.ComputeFuelCostNaive));
        Assert.Equal((5U, 168U), Solution.FindOptimalPoint(values, Solution.ComputeFuelCostCorrect));
    }
}
