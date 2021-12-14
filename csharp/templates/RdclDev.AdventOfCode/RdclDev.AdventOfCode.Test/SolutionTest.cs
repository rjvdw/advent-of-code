using System.Collections.Generic;
using Xunit;

namespace RdclDev.AdventOfCode.Test;

public class SolutionTest
{
    [Fact]
    public void Test1()
    {
        var lines = new List<string> { "Line1", "Line2" };
        Assert.Equal("Hello, World! [Line1, Line2]", Solution.Solve(lines));
    }
}