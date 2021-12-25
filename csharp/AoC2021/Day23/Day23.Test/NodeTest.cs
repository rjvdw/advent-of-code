using Xunit;

namespace Day23.Test;

public class NodeTest
{
    [Fact]
    public void TestIsSideRoom()
    {
        Assert.False(new Node(1, 6).IsSideRoom());
        Assert.True(new Node(2, 5).IsSideRoom());
        Assert.True(new Node(3, 5).IsSideRoom());
        Assert.True(new Node(4, 5).IsSideRoom());
        Assert.True(new Node(5, 5).IsSideRoom());
    }

    [Fact]
    public void TestDistanceTo()
    {
        var nodeA = new Node(2, 3);
        var nodeB = new Node(2, 5);
        var nodeC = new Node(1, 4);

        Assert.Equal(0, nodeA.DistanceTo(nodeA));
        Assert.Equal(4, nodeA.DistanceTo(nodeB));
        Assert.Equal(2, nodeA.DistanceTo(nodeC));

        Assert.Equal(4, nodeB.DistanceTo(nodeA));
        Assert.Equal(0, nodeB.DistanceTo(nodeB));
        Assert.Equal(2, nodeB.DistanceTo(nodeC));

        Assert.Equal(2, nodeC.DistanceTo(nodeA));
        Assert.Equal(2, nodeC.DistanceTo(nodeB));
        Assert.Equal(0, nodeC.DistanceTo(nodeC));
    }

    [Fact]
    public void TestImplicitConversionFromTuple()
    {
        var expected = new Node(2, 3);
        Node actual = (2, 3);

        Assert.Equal(expected, actual);
    }
}