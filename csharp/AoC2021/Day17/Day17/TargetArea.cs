namespace Day17;

public class TargetArea
{
    private readonly long _xMin;
    private readonly long _xMax;
    private readonly long _yMin;
    private readonly long _yMax;

    public long FindMaxHeight()
    {
        var (d, y) = GetYBound();
        return y switch
        {
            < 0 => d * (d - 1) / 2,
            > 0 => d * (d + 1) / 2,
            _ => throw new InvalidOperationException(
                "The target area contains a point with y=0. This means the trajectory can be arbitrarily high."),
        };
    }

    public IEnumerable<(long, long)> FindAllValidTrajectories()
    {
        var trajectories = new List<(long, long)>();
        var (d, _) = GetYBound();

        for (var x = 0; x <= _xMax; x += 1)
        {
            for (var y = -d; y < d; y += 1)
            {
                if (IsValidTrajectory((x, y)))
                    trajectories.Add((x, y));
            }
        }

        return trajectories;
    }

    private (long Distance, long Y) GetYBound()
    {
        if (_yMin < 0 && _yMax < 0)
            return (Math.Abs(_yMin), _yMin);

        if (_yMin > 0 && _yMax > 0)
            return (Math.Abs(_yMax), _yMax);

        return (0, 0);
    }

    private bool IsValidTrajectory((long VX, long VY) initial)
    {
        var (vX, vY) = initial;
        var x = 0L;
        var y = 0L;

        while (true)
        {
            x += vX;
            y += vY;

            if (x > _xMax) // we have overshot the target area
                return false;

            if (vY < 0 && y < _yMin) // we have overshot the target area
                return false;

            if (x >= _xMin && y >= _yMin && y <= _yMax)
                return true;

            if (vX == 0 && x < _xMin) // x will no longer change, so we will never reach the target area
                return false;

            if (vX != 0)
                vX -= vX / Math.Abs(vX);
            vY -= 1;
        }
    }

    private TargetArea(long xMin, long xMax, long yMin, long yMax)
    {
        _xMin = xMin;
        _xMax = xMax;
        _yMin = yMin;
        _yMax = yMax;
    }

    public static TargetArea Parse(IEnumerable<string> lines)
    {
        var line = lines.First();

        // find relevant indices (correct input is assumed, so no -1 checks are done)
        var i0 = 2 + line.IndexOf("x=", StringComparison.Ordinal);
        var i1 = line.IndexOf("..", i0, StringComparison.Ordinal);
        var i2 = i1 + 2;
        var i3 = line.IndexOf(", y=", i2, StringComparison.Ordinal);
        var i4 = i3 + 4;
        var i5 = line.IndexOf("..", i4, StringComparison.Ordinal);
        var i6 = i5 + 2;

        // parse values
        var xMin = long.Parse(line[i0..i1]);
        var xMax = long.Parse(line[i2..i3]);
        var yMin = long.Parse(line[i4..i5]);
        var yMax = long.Parse(line[i6..]);

        return new TargetArea(xMin, xMax, yMin, yMax);
    }
}
