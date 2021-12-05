using Xunit;

namespace Day04;

public class ValidHeightAttributeTest
{
    [Fact]
    public void TestValidHeights()
    {
        var attr = new ValidHeightAttribute
        {
            MinCm = 150,
            MaxCm = 193,
            MinInch = 59,
            MaxInch = 76
        };

        Assert.True(attr.IsValid("60in"));
        Assert.True(attr.IsValid("190cm"));
        Assert.False(attr.IsValid("190in"));
        Assert.False(attr.IsValid("190"));
    }
}
