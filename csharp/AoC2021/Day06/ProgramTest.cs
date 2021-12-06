using Xunit;

namespace Day06;

public class ProgramTest
{
    [Fact]
    public void TestSolve()
    {
        var values = new List<ulong> { 3, 4, 3, 1, 2 };

        Assert.Equal((ulong)5L, Program.Solve(values, 0));
        Assert.Equal((ulong)5L, Program.Solve(values, 1));
        Assert.Equal((ulong)6L, Program.Solve(values, 2));
        Assert.Equal((ulong)7L, Program.Solve(values, 3));
        Assert.Equal((ulong)9L, Program.Solve(values, 4));
        Assert.Equal((ulong)10L, Program.Solve(values, 5));
        Assert.Equal((ulong)5934L, Program.Solve(values, 80));
        Assert.Equal((ulong)26984457539L, Program.Solve(values, 256));
    }
}
