using Xunit;

namespace Day06;

public class ProgramTest
{
    [Fact]
    public void TestSolve()
    {
        var values = new List<ulong> { 3, 4, 3, 1, 2 };

        Assert.Equal(5UL, Program.Solve(values, 0));
        Assert.Equal(5UL, Program.Solve(values, 1));
        Assert.Equal(6UL, Program.Solve(values, 2));
        Assert.Equal(7UL, Program.Solve(values, 3));
        Assert.Equal(9UL, Program.Solve(values, 4));
        Assert.Equal(10UL, Program.Solve(values, 5));
        Assert.Equal(5934UL, Program.Solve(values, 80));
        Assert.Equal(26984457539UL, Program.Solve(values, 256));
    }
}
