namespace Day23;

public record Burrow(List<Amphipod> Amphipods, int SideRoomDepth)
{
    /*private*/public static readonly int[] HallwayX = { 1, 2, 4, 6, 8, 10, 11 };

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

    /*private*/public bool IsOccupied(Node node) => Amphipods.Any(a => a.Location == node);

    /*private*/public bool TryGetOccupant(Node node, out Amphipod occupant)
    {
        occupant = null!;
        var amphipod = Amphipods.Find(a => a.Location == node);

        if (amphipod is null)
            return false;

        occupant = amphipod;
        return true;
    }

    /*private*/public bool CanLeaveSideRoom(Amphipod amphipod) =>
        amphipod.Location.Y <= 2 || !IsOccupied((amphipod.Location.Y - 1, amphipod.Location.X));

    /*private*/public bool TryFindPlaceInSideRoom(Amphipod amphipod, out Node node)
    {
        var x = amphipod.Home;
        var y = SideRoomDepth + 1;
        node = null!;

        while (y > 1)
        {
            if (TryGetOccupant((y, x), out var occupant))
            {
                if (occupant.Color != amphipod.Color)
                    return false;

                y -= 1;
            }
            else
            {
                node = (y, x);
                return true;
            }
        }

        return false;
    }

    /*private*/public bool PathThroughHallwayIsFree(Amphipod amphipod, Node to)
    {
        var min = Math.Min(amphipod.Location.X, to.X);
        var max = Math.Max(amphipod.Location.X, to.X);
        for (var x = min; x <= max; x += 1)
        {
            var node = new Node(1, x);
            if (node != amphipod.Location && IsOccupied(node))
                return false;
        }

        return true;
    }

    /*private*/public (Burrow, int) WithUpdatedAmphipod(int idx, Amphipod amphipod, Node to)
    {
        var cost = amphipod.ComputeCost(to);
        var amphipods = new List<Amphipod>(Amphipods) { [idx] = amphipod.WithLocation(to) };
        amphipods.Sort();
        var nextState = new Burrow(amphipods, SideRoomDepth);

        return (nextState, cost);
    }

    /*private*/public bool CreatesDeadlock(Amphipod amphipod, Node to)
    {
        foreach (var other in Amphipods)
        {
            if (other.IsInSideRoom())
                continue;

            var aX = to.X;
            var aSideRoom = amphipod.Home;

            var bX = other.Location.X;
            var bSideRoom = other.Home;

            var aBlocksB = (bX < aX && aX < bSideRoom) || (bX > aX && aX > bSideRoom);
            var bBlocksA = (aX < bX && bX < aSideRoom) || (aX > bX && bX > aSideRoom);

            if (aBlocksB && bBlocksA)
                return true;
        }

        return false;
    }
}
