using System.Collections.Generic;
using Xunit;

namespace Day25.Test;

public class SolutionTest
{
    [Fact]
    public void TestSolve()
    {
        var sf = SeaFloor.Parse(new List<string>
        {
            "v...>>.vv>",
            ".vv>>.vv..",
            ">>.>v>...v",
            ">>v>>.>.v.",
            "v>v.vv.v..",
            ">.>>..v...",
            ".vv..>.>v.",
            "v.v..>>v.v",
            "....v..v.>",
        });
        var i = 0;
        while (sf.TryNext(out var next))
        {
            i += 1;
            sf = next;
        }

        Assert.Equal(57, i);
    }
}
