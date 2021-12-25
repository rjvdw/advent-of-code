namespace Day23;

public record Burrow(List<Amphipod> Amphipods, int SideRoomDepth)
{
    private static readonly int[] HallwayX = { 1, 2, 4, 6, 8, 10, 11 };

    public bool IsFinished() => Amphipods.All(a => a.IsHome());

    public int ComputePriority() => Amphipods.Count(a => !a.Exhausted);

    public int MinimumRemainingCost() => Amphipods
        .Where(amphipod => !amphipod.IsHome())
        .Sum(amphipod => amphipod.ComputeCost((2, amphipod.Home)));

    public bool TryFindMoveToSideRoom(out Burrow burrow, out int cost)
    {
        var idx = -1;
        foreach (var amphipod in Amphipods)
        {
            idx += 1;
            if (amphipod.Exhausted || amphipod.IsHome() || !CanLeaveSideRoom(amphipod))
                continue;

            if (TryFindPlaceInSideRoom(amphipod, out var node))
            {
                if (PathThroughHallwayIsFree(amphipod, node))
                {
                    (burrow, cost) = WithUpdatedAmphipod(idx, amphipod, node);
                    return true;
                }
            }
        }

        burrow = null!;
        cost = 0;
        return false;
    }

    public IEnumerable<(Burrow Next, int Cost)> FindMovesToHallway()
    {
        var moves = new List<(Burrow, int)>();
        var idx = -1;
        foreach (var amphipod in Amphipods)
        {
            idx += 1;
            if (amphipod.Exhausted || !amphipod.IsInSideRoom() || !CanLeaveSideRoom(amphipod))
                continue;

            var neighbours = HallwayX
                .Select(x => new Node(1, x))
                .Where(node => PathThroughHallwayIsFree(amphipod, node))
                .Where(node => !CreatesDeadlock(amphipod, node));

            moves.AddRange(neighbours.Select(neighbour => WithUpdatedAmphipod(idx, amphipod, neighbour)));
        }

        return moves;
    }

    private bool CanLeaveSideRoom(Amphipod amphipod)
    {
        throw new NotImplementedException("TODO");
    }

    private bool TryFindPlaceInSideRoom(Amphipod amphipod, out Node node)
    {
        throw new NotImplementedException("TODO");
    }

    private bool PathThroughHallwayIsFree(Amphipod amphipod, Node to)
    {
        throw new NotImplementedException("TODO");
    }

    private (Burrow, int) WithUpdatedAmphipod(int idx, Amphipod amphipod, Node node)
    {
        throw new NotImplementedException("TODO");
    }

    private bool CreatesDeadlock(Amphipod amphipod, Node to)
    {
        throw new NotImplementedException("TODO");
    }
}
