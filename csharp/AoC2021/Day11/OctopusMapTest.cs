using Xunit;

namespace Day11;

public class OctopusMapTest
{
    [Fact]
    public void TestTickSimple()
    {
        var map = OctopusMap.Parse(new[]
        {
            "11111",
            "19991",
            "19191",
            "19991",
            "11111"
        });

        var expected1 = string.Join('\n',
            "34543",
            "40004",
            "50005",
            "40004",
            "34543");

        var expected2 = string.Join('\n',
            "45654",
            "51115",
            "61116",
            "51115",
            "45654");

        var (next1, nrFlashes1) = map.Tick();
        Assert.Equal(9, nrFlashes1);
        Assert.Equal(expected1, next1.ToString());

        var (next2, nrFlashes2) = next1.Tick();
        Assert.Equal(0, nrFlashes2);
        Assert.Equal(expected2, next2.ToString());
    }
}
