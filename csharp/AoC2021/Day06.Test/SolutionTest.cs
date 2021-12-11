using System.Collections.Generic;
using Xunit;

namespace Day06.Test;

public class SolutionTest
{
    [Fact]
    public void TestSolve()
    {
        var values = new List<ulong> { 3, 4, 3, 1, 2 };

        Assert.Equal(5UL, Solution.Solve(values, 0));
        Assert.Equal(5UL, Solution.Solve(values, 1));
        Assert.Equal(6UL, Solution.Solve(values, 2));
        Assert.Equal(7UL, Solution.Solve(values, 3));
        Assert.Equal(9UL, Solution.Solve(values, 4));
        Assert.Equal(10UL, Solution.Solve(values, 5));
        Assert.Equal(5934UL, Solution.Solve(values, 80));
        Assert.Equal(26984457539UL, Solution.Solve(values, 256));
    }
}
