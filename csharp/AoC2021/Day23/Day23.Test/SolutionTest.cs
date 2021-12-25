using System.Collections.Generic;
using Xunit;

namespace Day23.Test;

public class SolutionTest
{
    [Fact]
    public void TestParseInput()
    {
        var input = new List<string>
        {
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#",
            "  #########",
        };

        var (expectedAmphipods, expectedSideRoomDepth) = TestData();
        var (actualAmphipods, actualSideRoomDepth) = Solution.Parse(input);

        Assert.Equal(expectedSideRoomDepth, actualSideRoomDepth);
        Assert.Equal(expectedAmphipods, actualAmphipods);
    }

    [Fact]
    public void TestFindCheapestPath()
    {
        var (amphipods, sideRoomDepth) = TestData();
        Assert.Equal(12521, Solution.FindCheapestPath(amphipods, sideRoomDepth));
    }

    private static (List<Amphipod>, int) TestData() => (
        new List<Amphipod>
        {
            Amphipod.Parse('B', (2, 3)),
            Amphipod.Parse('C', (2, 5)),
            Amphipod.Parse('B', (2, 7)),
            Amphipod.Parse('D', (2, 9)),
            Amphipod.Parse('A', (3, 3)),
            Amphipod.Parse('D', (3, 5)),
            Amphipod.Parse('C', (3, 7)),
            Amphipod.Parse('A', (3, 9)),
        },
        2
    );
}
