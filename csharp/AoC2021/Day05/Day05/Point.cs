namespace Day05;

public struct Point
{
    public int X { get; set; }
    public int Y { get; set; }

    public Point(int x, int y)
    {
        X = x;
        Y = y;
    }

    public override bool Equals(object? obj) => obj is Point p && p.X == X && p.Y == Y;
    public static bool operator ==(Point left, Point right) => left.Equals(right);
    public static bool operator !=(Point left, Point right) => !(left == right);

    public override int GetHashCode() => HashCode.Combine(X, Y);

    public static Point Parse(string s)
    {
        var p = s.IndexOf(',');
        if (p == -1) throw new ArgumentException("Invalid input.", nameof(s));
        return new Point(int.Parse(s[..p]), int.Parse(s[(p + 1)..]));
    }
}
