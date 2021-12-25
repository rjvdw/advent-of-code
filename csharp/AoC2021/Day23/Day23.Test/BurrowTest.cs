using System.Collections.Generic;
using Xunit;

namespace Day23.Test;

public class BurrowTest
{
    [Fact]
    public void TestIsFinished()
    {
        Assert.True(FinishedBurrow().IsFinished());
        Assert.False(UnfinishedBurrow().IsFinished());
    }

    [Fact]
    public void TestMinimumRemainingCost()
    {
        Assert.Equal(9302, UnfinishedBurrow().MinimumRemainingCost());
    }

    [Fact]
    public void TestIsOccupied()
    {
        var burrow = UnfinishedBurrow();

        Assert.True(burrow.IsOccupied((3, 3)));
        Assert.False(burrow.IsOccupied((1, 1)));
    }

    [Fact]
    public void TestTryGetOccupant()
    {
        var burrow = UnfinishedBurrow();
        var c = Amphipod.Parse('C', (3, 3));

        Assert.True(burrow.TryGetOccupant((3, 3), out var occupant));
        Assert.Equal(c, occupant);

        Assert.False(burrow.TryGetOccupant((1, 1), out _));
    }

    [Fact]
    public void TestCanLeaveSideRoom()
    {
        var burrow = UnfinishedBurrow();
        var c = Amphipod.Parse('C', (3, 3));
        var d = Amphipod.Parse('D', (2, 3));

        Assert.False(burrow.CanLeaveSideRoom(c));
        Assert.True(burrow.CanLeaveSideRoom(d));
    }

    [Fact]
    public void TryFindPlaceInSideRoom()
    {
        var burrow = InProgressBurrow1();
        var a = Amphipod.Parse('A', (2, 5));
        var b = Amphipod.Parse('B', (3, 7));

        Assert.True(burrow.TryFindPlaceInSideRoom(a, out var node));
        Assert.Equal(new Node(3, 3), node);

        Assert.False(burrow.TryFindPlaceInSideRoom(b, out _));
    }

    [Fact]
    public void TestPathThroughHallwayIsFree()
    {
        var burrow = InProgressBurrow2();
        var a = Amphipod.Parse('A', (2, 5));

        Assert.True(burrow.PathThroughHallwayIsFree(a, (1, 2)));
        Assert.False(burrow.PathThroughHallwayIsFree(a, (1, 11)));
    }

    [Fact]
    public void TestCreatesDeadlock()
    {
        var burrow = InProgressBurrow2();
        var b = Amphipod.Parse('B', (2, 7));

        Assert.True(burrow.CreatesDeadlock(b, (1, 8)));
        Assert.False(burrow.CreatesDeadlock(b, (1, 11)));
    }

    [Fact]
    public void WithUpdatedAmphipod()
    {
        var burrow = InProgressBurrow2();
        var expected = new Burrow(
            new List<Amphipod>
            {
                Amphipod.Parse('C', (1, 1)),
                Amphipod.Parse('A', (1, 2)),
                Amphipod.Parse('D', (1, 6)),
                Amphipod.Parse('B', (2, 7)),
                Amphipod.Parse('A', (2, 9)),
                Amphipod.Parse('C', (3, 5)),
                Amphipod.Parse('B', (3, 7)),
                Amphipod.Parse('D', (3, 9)),
            },
            2
        );
        var a = Amphipod.Parse('A', (2, 5));

        var (next, cost) = burrow.WithUpdatedAmphipod(2, a, (1, 2));

        Assert.Equal(4, cost);
        Assert.Equal(expected, next);
    }

    private static Burrow FinishedBurrow() => new(
        new List<Amphipod>
        {
            Amphipod.Parse('A', (2, 3)),
            Amphipod.Parse('B', (2, 5)),
            Amphipod.Parse('C', (2, 7)),
            Amphipod.Parse('D', (2, 9)),
            Amphipod.Parse('A', (3, 3)),
            Amphipod.Parse('B', (3, 5)),
            Amphipod.Parse('C', (3, 7)),
            Amphipod.Parse('D', (3, 9)),
        },
        2
    );

    private static Burrow UnfinishedBurrow() => new(
        new List<Amphipod>
        {
            Amphipod.Parse('D', (2, 3)),
            Amphipod.Parse('A', (2, 5)),
            Amphipod.Parse('B', (2, 7)),
            Amphipod.Parse('A', (2, 9)),
            Amphipod.Parse('C', (3, 3)),
            Amphipod.Parse('C', (3, 5)),
            Amphipod.Parse('B', (3, 7)),
            Amphipod.Parse('D', (3, 9)),
        },
        2
    );

    private static Burrow InProgressBurrow1() => new(
        new List<Amphipod>
        {
            Amphipod.Parse('D', (1, 1)),
            Amphipod.Parse('C', (1, 2)),
            Amphipod.Parse('A', (2, 5)),
            Amphipod.Parse('B', (2, 7)),
            Amphipod.Parse('A', (2, 9)),
            Amphipod.Parse('C', (3, 5)),
            Amphipod.Parse('B', (3, 7)),
            Amphipod.Parse('D', (3, 9)),
        },
        2
    );

    private static Burrow InProgressBurrow2() => new(
        new List<Amphipod>
        {
            Amphipod.Parse('C', (1, 1)),
            Amphipod.Parse('D', (1, 6)),
            Amphipod.Parse('A', (2, 5)),
            Amphipod.Parse('B', (2, 7)),
            Amphipod.Parse('A', (2, 9)),
            Amphipod.Parse('C', (3, 5)),
            Amphipod.Parse('B', (3, 7)),
            Amphipod.Parse('D', (3, 9)),
        },
        2
    );
}
