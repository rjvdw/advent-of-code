using Xunit;

namespace Day05.Test;

public class PointTest
{
    [Fact]
    public void TestEquals()
    {
        var a = new Point(1, 2);
        var b = new Point(1, 2);
        var c = new Point(1, 1);
        var d = new Point(2, 2);
        var e = new Point(2, 1);

        Assert.True(a.Equals(b));
        Assert.True(a == b);
        Assert.False(a != b);

        Assert.False(a.Equals(c));
        Assert.False(a == c);
        Assert.True(a != c);

        Assert.False(a.Equals(d));
        Assert.False(a == d);
        Assert.True(a != d);

        Assert.False(a.Equals(e));
        Assert.False(a == e);
        Assert.True(a != e);
    }
}
