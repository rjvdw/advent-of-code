namespace Day23;

public record Node(int Y, int X)
{
    public bool IsSideRoom() => Y > 1;

    public int DistanceTo(Node other) =>
        Y == 1 || other.Y == 1 || X == other.X
            ? Math.Abs(X - other.X) + Math.Abs(Y - other.Y)
            : Math.Abs(X - other.X) + Math.Abs(Y - 1) + Math.Abs(1 - other.Y);

    public static implicit operator Node((int Y, int X) n) => new(n.Y, n.X);
}
