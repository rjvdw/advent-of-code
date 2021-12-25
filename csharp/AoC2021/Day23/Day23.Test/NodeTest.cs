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

    [Fact]
    public void TestCompareTo()
    {
        // y equal, x equal
        Assert.Equal(0, new Node(1, 1).CompareTo(new Node(1, 1)));

        // y equal, x greater
        Assert.True(new Node(1, 2).CompareTo(new Node(1, 1)) > 0);

        // y equal, x less
        Assert.True(new Node(1, 0).CompareTo(new Node(1, 1)) < 0);

        // y greater
        Assert.True(new Node(2, 1).CompareTo(new Node(1, 1)) > 0);

        // y less
        Assert.True(new Node(0, 1).CompareTo(new Node(1, 1)) < 0);
    }
}
