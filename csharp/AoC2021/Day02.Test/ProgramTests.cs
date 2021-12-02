using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day02.Test;

public class ProgramTests
{
    [Fact]
    public void TestFollowInstructions()
    {
        var instructions = new List<string>
            {
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2"
            }
            .Select(Instruction.Parse)
            .ToList();

        Assert.Equal((10, 15), Program.FollowInstructions(instructions));
    }

    [Fact]
    public void TestFollowInstructionsWithAim()
    {
        var instructions = new List<string>
            {
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2"
            }
            .Select(Instruction.Parse)
            .ToList();

        Assert.Equal((60, 15), Program.FollowInstructionsWithAim(instructions));
    }
}
