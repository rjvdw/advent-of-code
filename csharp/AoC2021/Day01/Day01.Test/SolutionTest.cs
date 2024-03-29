using System.Collections.Generic;
using Xunit;

namespace Day01.Test;

public class SolutionTest
{
    [Fact]
    public void TestCountIncreasesWithWindow1()
    {
        var numbers = new List<int> { 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 };
        Assert.Equal(7, Solution.CountIncreases(numbers, 1));
    }

    [Fact]
    public void TestCountIncreasesWithWindow3()
    {
        var numbers = new List<int> { 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 };
        Assert.Equal(5, Solution.CountIncreases(numbers, 3));
    }
}
