namespace Day22;

public record Cuboid(bool IsOn, Range XRange, Range YRange, Range ZRange)
{
    public bool FitsWithin(Cuboid other) =>
        XRange.FitsWithin(other.XRange)
        && YRange.FitsWithin(other.YRange)
        && ZRange.FitsWithin(other.ZRange);

    public long Size() => XRange.Size() * YRange.Size() * ZRange.Size();

    /// <summary>
    /// If this cuboid has overlap with the other cuboid, return a selection of cuboids that together contain all cubes
    /// that are in this cuboid, but not in the second.
    /// </summary>
    /// <param name="other">The cuboid to compare with.</param>
    /// <returns>Cuboids containing the cubes that are not in the other cuboids.</returns>
    public List<Cuboid>? Subtract(Cuboid other)
    {
        var subs = new List<Cuboid>();

        var partitionX = XRange.Partition(other.XRange);
        var partitionY = YRange.Partition(other.YRange);
        var partitionZ = ZRange.Partition(other.ZRange);

        if (partitionX == null || partitionY == null || partitionZ == null)
            return null;

        foreach (var xRange in partitionX)
        {
            foreach (var yRange in partitionY)
            {
                foreach (var zRange in partitionZ)
                {
                    var sub = new Cuboid(
                        IsOn,
                        xRange,
                        yRange,
                        zRange
                    );

                    if (!sub.FitsWithin(other))
                        subs.Add(sub);
                }
            }
        }

        return subs;
    }

    public static Cuboid Parse(string s)
    {
        var i = FindIndices(s);
        if (i is null)
            throw new FormatException($"Invalid input: {s}");

        return new Cuboid(
            s.StartsWith("on"),
            Range.Parse(s[i[0]..i[1]]),
            Range.Parse(s[i[2]..i[3]]),
            Range.Parse(s[i[4]..])
        );
    }

    private static int[]? FindIndices(string s)
    {
        var i = new[] { 0, 0, 0, 0, 0 };

        // XRange.From
        i[0] = s.IndexOf("x=", StringComparison.Ordinal);
        if (i[0] == -1) return null;
        i[0] += 2;

        // XRange.To
        i[1] = s.IndexOf(",y=", i[0], StringComparison.Ordinal);
        if (i[1] == -1) return null;

        // YRange.From
        i[2] = i[1] + 3;

        // YRange.To
        i[3] = s.IndexOf(",z=", i[2], StringComparison.Ordinal);
        if (i[3] == -1) return null;

        // ZRange.From
        i[4] = i[3] + 3;

        return i;
    }
}
