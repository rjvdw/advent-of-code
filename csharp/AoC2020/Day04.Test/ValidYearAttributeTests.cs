using Xunit;

namespace Day04.Test;

public class ValidYearAttributeTests
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
