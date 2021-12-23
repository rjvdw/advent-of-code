namespace Day23;

public record Candidate(List<(Node Node, Amphipod Amphipod)> Amphipods, List<bool> Exhausted, int BurrowDepth)
{
    private static readonly int[] _hallwayX = { 1, 2, 4, 6, 8, 10, 11 };

    public Candidate Normalized()
    {
        var amphipods = new List<(Node Node, Amphipod Amphipod)>(Amphipods);
        amphipods.Sort((one, other) =>
        {
            var result = one.Node.Y.CompareTo(other.Node.Y);
            return result == 0 ? one.Node.X.CompareTo(other.Node.X) : result;
        });

        var exhausted = Enumerable.Repeat(false, amphipods.Count).ToList();
        for (var i = 0; i < amphipods.Count; i += 1)
        {
            var item = amphipods[i];
            var amphipod = item.Amphipod;
            exhausted[i] = Exhausted[amphipod.Index];
            item.Amphipod = amphipod.WithIndex(i);
        }

        return new Candidate(amphipods, exhausted, BurrowDepth);
    }

    public bool IsDone() => Amphipods.All(v => v.Node.X == v.Amphipod.TargetBurrow);

    public bool ExitIsBlocked(Node node)
    {
        for (var y = 2; y < node.Y; y += 1)
            if (Amphipods.ContainsNode(new Node(y, node.X)))
                return true;

        return false;
    }

    public Node? FindMoveToBurrow(Node node, Amphipod amphipod)
    {
        if (node.IsBurrow() && ExitIsBlocked(node))
            return null;

        var burrow = amphipod.TargetBurrow;
        var room = new Node(2, burrow);

        if (!PathExists(node, room))
            return null;

        if (Amphipods.GetNode(room) is null)
            return null;

        var bottomRoom = new Node(BurrowDepth + 1, burrow);
        var targetRoom = bottomRoom;
        while (room.Y <= BurrowDepth)
        {
            room = new Node(room.Y + 1, room.X);
            var other = Amphipods.GetNode(room);
            if (other is not null)
            {
                if (amphipod.Color == other.Color)
                {
                    if (targetRoom.Y >= room.Y)
                        targetRoom = new Node(room.Y - 1, targetRoom.X);
                }
                else
                    return null;
            }
        }

        return targetRoom;
    }

    public List<Node> FindMovesToHallway(Node node, Amphipod amphipod)
    {
        if (!node.IsBurrow())
            return new List<Node>();

        return _hallwayX
            .Select(x => new Node(1, x))
            .Where(n => !Amphipods.ContainsNode(n))
            .Where(n => PathExists(node, n))
            .Where(n => WontBlock(n, amphipod))
            .ToList();
    }

    private bool PathExists(Node from, Node to)
    {
        var min = Math.Min(from.X, to.X);
        var max = Math.Max(from.X, to.X);

        for (var x = min; x <= max; x += 1)
        {
            var node = new Node(1, x);
            if (node != from && Amphipods.ContainsNode(node))
                return false;
        }

        return true;
    }

    private bool WontBlock(Node node, Amphipod amphipod)
    {
        foreach (var (otherNode, otherAmphipod) in Amphipods)
        {
            if (otherNode.Y != 1)
                continue;

            var aX = node.X;
            var aBurrow = amphipod.TargetBurrow;

            var bX = otherNode.X;
            var bBurrow = otherAmphipod.TargetBurrow;

            var aBlocksB = (bX < aX && aX < bBurrow) || (bX > aX && aX > bBurrow);
            var bBlocksA = (aX < bX && bX < aBurrow) || (aX > bX && bX > aBurrow);

            if (aBlocksB && bBlocksA)
                return false;
        }

        return true;
    }
}
