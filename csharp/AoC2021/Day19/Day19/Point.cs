namespace Day19;

public record Point(long X, long Y, long Z)
{
    public const ushort Orientations = 24;

    public Point Rotate(ushort orientation) => orientation switch
    {
        0 => this,
        1 => new Point(X, Z, -Y),
        2 => new Point(X, -Z, Y),
        3 => new Point(X, -Y, -Z),
        4 => new Point(Y, X, -Z),
        5 => new Point(Y, Z, X),
        6 => new Point(Y, -Z, -X),
        7 => new Point(Y, -X, Z),
        8 => new Point(Z, X, Y),
        9 => new Point(Z, Y, -X),
        10 => new Point(Z, -Y, X),
        11 => new Point(Z, -X, -Y),
        12 => new Point(-Z, X, -Y),
        13 => new Point(-Z, Y, X),
        14 => new Point(-Z, -Y, -X),
        15 => new Point(-Z, -X, Y),
        16 => new Point(-Y, X, Z),
        17 => new Point(-Y, Z, -X),
        18 => new Point(-Y, -Z, X),
        19 => new Point(-Y, -X, -Z),
        20 => new Point(-X, Y, -Z),
        21 => new Point(-X, Z, Y),
        22 => new Point(-X, -Z, -Y),
        _ => new Point(-X, -Y, Z),
    };

    public long DistanceTo(Point other) => Math.Abs(X - other.X) + Math.Abs(Y - other.Y) + Math.Abs(Z - other.Z);

    public static Point operator +(Point a, Point b) => new(a.X + b.X, a.Y + b.Y, a.Z + b.Z);

    public static Point operator -(Point a, Point b) => new(a.X - b.X, a.Y - b.Y, a.Z - b.Z);

    public static Point Parse(string s)
    {
        var i0 = s.IndexOf(',');
        if (i0 != -1)
        {
            var i1 = s.IndexOf(',', i0 + 1);
            if (i1 != -1)
            {
                return new Point(
                    long.Parse(s[..i0]),
                    long.Parse(s[(i0 + 1)..i1]),
                    long.Parse(s[(i1 + 1)..])
                );
            }
        }

        throw new FormatException($"Invalid line: {s}.");
    }
}
