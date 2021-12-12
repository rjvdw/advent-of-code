namespace Day05;

public readonly struct Line
{
    private const string Separator = " -> ";

    private Point P1 { get; }
    private Point P2 { get; }

    private Line(Point p1, Point p2)
    {
        P1 = p1;
        P2 = p2;
    }

    public bool IsHorizontal() => P1.Y == P2.Y;

    public bool IsVertical() => P1.X == P2.X;

    public IEnumerable<Point> GetPoints()
    {
        var point = P1;
        yield return point;
        while (!point.Equals(P2))
        {
            if (point.X < P2.X) point.X += 1;
            else if (point.X > P2.X) point.X -= 1;

            if (point.Y < P2.Y) point.Y += 1;
            else if (point.Y > P2.Y) point.Y -= 1;

            yield return point;
        }
    }

    public override string ToString() => $"{P1}{Separator}{P2}";

    public static Line Parse(string s)
    {
        var p = s.IndexOf(Separator, StringComparison.Ordinal);
        if (p == -1) throw new ArgumentException("Invalid input.", nameof(s));
        return new Line(Point.Parse(s[..p]), Point.Parse(s[(p + Separator.Length)..]));
    }
}
