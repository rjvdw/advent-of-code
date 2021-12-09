using Xunit;

namespace Day09;

public class ProgramTest
{
    [Fact]
    public void TestCalculateRisk()
    {
        Assert.Equal(15u, Program.CalculateRisk(new List<byte> { 1, 0, 5, 5 }));
    }
}
