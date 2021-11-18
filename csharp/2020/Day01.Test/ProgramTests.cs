using System.Collections.Generic;
using Xunit;

namespace Day01.Test
{
    public class ProgramTests
    {
        [Fact]
        public void TestPart1()
        {
            var numbers = new List<int> { 1721, 979, 366, 299, 675, 1456 };
            Assert.Equal(514579, Program.SolvePart1(numbers, 2020));
        }

        [Fact]
        public void TestPart2()
        {
            var numbers = new List<int> { 1721, 979, 366, 299, 675, 1456 };
            Assert.Equal(241861950, Program.SolvePart2(numbers, 2020));
        }
    }
}
