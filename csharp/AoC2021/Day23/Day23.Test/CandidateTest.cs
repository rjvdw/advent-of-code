using System.Collections.Generic;
using Xunit;

namespace Day23.Test;

public class CandidateTest
{
    [Fact]
    public void TestIsDone()
    {
        Assert.True(GetDoneCandidate().IsDone());
    }

    [Fact]
    public void TestNotIsDone()
    {
        Assert.False(GetCandidate().IsDone());
    }

    [Fact]
    public void TestExitIsBlocked()
    {
        var candidate = GetCandidate();

        Assert.True(candidate.ExitIsBlocked(new Node(3, 3)));
        Assert.False(candidate.ExitIsBlocked(new Node(2, 3)));
    }

    private static Candidate GetDoneCandidate() => new(
        new List<(Node Node, Amphipod Amphipod)>
        {
            (new Node(2, 3), new Amphipod(Color.Amber, 0)),
            (new Node(3, 3), new Amphipod(Color.Amber, 1)),
            (new Node(2, 5), new Amphipod(Color.Bronze, 2)),
            (new Node(3, 5), new Amphipod(Color.Bronze, 3)),
            (new Node(2, 7), new Amphipod(Color.Copper, 4)),
            (new Node(3, 7), new Amphipod(Color.Copper, 5)),
            (new Node(2, 9), new Amphipod(Color.Desert, 6)),
            (new Node(3, 9), new Amphipod(Color.Desert, 7)),
        },
        new List<bool>(),
        2
    );

    private static Candidate GetCandidate() => new(
        new List<(Node Node, Amphipod Amphipod)>
        {
            (new Node(2, 3), new Amphipod(Color.Desert, 0)),
            (new Node(3, 3), new Amphipod(Color.Copper, 1)),
            (new Node(2, 5), new Amphipod(Color.Amber, 2)),
            (new Node(3, 5), new Amphipod(Color.Bronze, 3)),
            (new Node(2, 7), new Amphipod(Color.Bronze, 4)),
            (new Node(3, 7), new Amphipod(Color.Copper, 5)),
            (new Node(2, 9), new Amphipod(Color.Amber, 6)),
            (new Node(3, 9), new Amphipod(Color.Desert, 7)),
        },
        new List<bool>(),
        2
    );
}
