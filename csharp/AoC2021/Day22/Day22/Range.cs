namespace Day22;

public record Range(long From, long To)
{
    public bool FitsWithin(Range other) => From >= other.From && To <= other.To;

    public long Size() => To - From + 1;

    /// <summary>
    /// Creates a partition, so that every cube in the partition is either completely disjoint from <code>other</code>,
    /// or is completely contained within <code>other</code>. If this range is already completely disjoint from
    /// <code>other</code> to begin with, this method will return <code>null</code>.
    /// </summary>
    /// <param name="other">The other range to use for the partitioning.</param>
    /// <returns>A partition of this range.</returns>
    public List<Range>? Partition(Range other)
    {
        if (To < other.From || From > other.To)
            return null;

        var partitions = new List<Range> { new(Math.Max(From, other.From), Math.Min(To, other.To)) };

        if (From < other.From)
            partitions.Add(new Range(From, other.From - 1));

        if (To > other.To)
            partitions.Add(new Range(other.To + 1, To));

        return partitions;
    }

    public static Range Parse(string s)
    {
        var i = s.IndexOf("..", StringComparison.Ordinal);
        if (i == -1) throw new FormatException($"Invalid range: {s}");
        var from = long.Parse(s[..i]);
        var to = long.Parse(s[(i + 2)..]);
        if (from > to)
            throw new FormatException($"Invalid range: {s}");
        return new Range(from, to);
    }
}
