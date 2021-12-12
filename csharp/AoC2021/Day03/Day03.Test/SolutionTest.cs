using System.Collections.Generic;
using Xunit;

namespace Day03.Test;

public class SolutionTest
{
    [Fact]
    public void TestComputeGammaAndEpsilonRate()
    {
        var values = new List<ushort>
        {
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010
        };
        Assert.Equal((22, 9), Solution.ComputeGammaAndEpsilonRate(values, 5));
    }

    [Fact]
    public void TestComputeLifeSupportRating()
    {
        var values = new List<ushort>
        {
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010
        };
        Assert.Equal(230, Solution.ComputeLifeSupportRating(values, 5));
    }
}
