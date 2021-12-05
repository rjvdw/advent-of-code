using Xunit;

namespace Day04;

public class ValidYearAttributeTest
{
    [Fact]
    public void TestValidYears()
    {
        var attr = new ValidYearAttribute
        {
            Min = 1920,
            Max = 2002
        };

        Assert.True(attr.IsValid("2002"));
        Assert.False(attr.IsValid("2003"));
    }
}
