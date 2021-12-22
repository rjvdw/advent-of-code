namespace Day22;

public record Cuboid(bool IsOn, Range XRange, Range YRange, Range ZRange)
{
    public bool IsDisjoint(Cuboid other) =>
        XRange.IsDisjoint(other.XRange)
        || YRange.IsDisjoint(other.YRange)
        || ZRange.IsDisjoint(other.ZRange);

    public bool FitsWithin(Cuboid other) =>
        XRange.FitsWithin(other.XRange)
        && YRange.FitsWithin(other.YRange)
        && ZRange.FitsWithin(other.ZRange);

    public long Size() => XRange.Size() * YRange.Size() * ZRange.Size();

    /// <summary>
    /// Return a selection of cuboids that together contain all cubes that are in <paramref name="one"/>, but not in
    /// <paramref name="other"/>.
    /// </summary>
    /// <param name="one">The cuboid from which cubes are subtracted.</param>
    /// <param name="other">The cuboid that will be subtracted.</param>
    /// <returns>Cuboids containing the cubes that are in <paramref name="one"/>, but not <paramref name="other"/>.</returns>
    public static IEnumerable<Cuboid> operator -(Cuboid one, Cuboid other) =>
        one.XRange.Partition(other.XRange)
            .SelectMany(x => one
                .YRange
                .Partition(other.YRange)
                .Select(y => (x, y))
            )
            .SelectMany(ranges => one
                .ZRange
                .Partition(other.ZRange)
                .Select(z => (ranges.x, ranges.y, z))
            )
            .Select(ranges => new Cuboid(one.IsOn, ranges.x, ranges.y, ranges.z))
            .Where(cuboid => !cuboid.FitsWithin(other));

    public static Cuboid Parse(string s)
    {
        var i = FindIndices(s);
        return new Cuboid(
            s.StartsWith("on"),
            Range.Parse(s[i[0]..i[1]]),
            Range.Parse(s[i[2]..i[3]]),
            Range.Parse(s[i[4]..])
        );
    }

    private static int[] FindIndices(string s)
    {
        var i = new[] { 0, 0, 0, 0, 0 };

        // XRange.From
        i[0] = s.IndexOf("x=", StringComparison.Ordinal);
        if (i[0] == -1) throw new FormatException($"Invalid input: {s}");
        i[0] += 2;

        // XRange.To
        i[1] = s.IndexOf(",y=", i[0], StringComparison.Ordinal);
        if (i[1] == -1) throw new FormatException($"Invalid input: {s}");

        // YRange.From
        i[2] = i[1] + 3;

        // YRange.To
        i[3] = s.IndexOf(",z=", i[2], StringComparison.Ordinal);
        if (i[3] == -1) throw new FormatException($"Invalid input: {s}");

        // ZRange.From
        i[4] = i[3] + 3;

        return i;
    }
}
