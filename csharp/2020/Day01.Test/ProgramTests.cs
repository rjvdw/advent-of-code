using System.Collections.Generic;
using Xunit;

namespace Day01.Test
{
    public class ProgramTests
    {
        [Fact]
        public void TestFindNumbers()
        {
            var numbers = new List<int> { 1721, 979, 366, 299, 675, 1456 };
            Assert.Equal(514579, Program.FindNumbers(numbers, 2020, 2));
            Assert.Equal(241861950, Program.FindNumbers(numbers, 2020, 3));
        }
    }
}
