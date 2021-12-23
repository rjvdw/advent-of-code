using System.Collections.Generic;
using Xunit;

namespace Day23.Test;

public class CandidateExtensionsTest
{
    [Fact]
    public void TestContainsNode()
    {
        var list = new List<(Node, Amphipod)>
        {
            (new Node(2, 1), new Amphipod(Color.Amber, 0)),
        };

        Assert.True(list.ContainsNode(new Node(2, 1)));
        Assert.False(list.ContainsNode(new Node(1, 2)));
    }

    [Fact]
    public void TestGetNode()
    {
        var list = new List<(Node, Amphipod)>
        {
            (new Node(2, 1), new Amphipod(Color.Amber, 0)),
        };

        Assert.Equal(new Amphipod(Color.Amber, 0), list.GetNode(new Node(2, 1)));
        Assert.Null(list.GetNode(new Node(1, 2)));
    }
}
