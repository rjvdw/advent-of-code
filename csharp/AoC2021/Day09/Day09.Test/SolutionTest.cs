using System.Collections.Generic;
using Xunit;

namespace Day09.Test;

public class SolutionTest
{
    [Fact]
    public void TestCalculateRisk()
    {
        Assert.Equal(15u, Solution.CalculateRisk(new List<byte> { 1, 0, 5, 5 }));
    }
}
