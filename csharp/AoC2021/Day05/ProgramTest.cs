using Xunit;

namespace Day05;

public class ProgramTest
{
    [Fact]
    public void TestCountDangerousPoints()
    {
        var lines = new List<string>
        {
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        }.Select(Line.Parse).ToList();
        Assert.Equal(5, Program.CountDangerousPoints(lines, false));
        Assert.Equal(12, Program.CountDangerousPoints(lines, true));
    }
}
