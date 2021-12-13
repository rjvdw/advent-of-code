using System;
using Xunit;

namespace Day02.Test;

public class InstructionTest
{
    [Fact]
    public void TestFailsOnInvalidInput()
    {
        Assert.Throws<ArgumentException>(() => Instruction.Parse("line_without_a_space"));
        Assert.Throws<ArgumentException>(() => Instruction.Parse("downwards 15"));
    }
}
