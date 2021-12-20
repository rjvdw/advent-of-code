namespace Day20;

public class Image
{
    private const ushort DarkRegion = 0b000_000_000;
    private const ushort LightRegion = 0b111_111_111;

    private readonly HashSet<ushort> _iea;
    private readonly HashSet<(long Row, long Col)> _lit;
    private readonly Bounds _bounds;
    private bool _defaultLit;

    private Image(HashSet<ushort> iea)
    {
        _iea = iea;
        _lit = new HashSet<(long Row, long Col)>();
        _bounds = new Bounds((long.MaxValue, long.MaxValue), (long.MinValue, long.MinValue));
        _defaultLit = false;
    }

    public (long NrLit, bool IsInfinite) CountLitPixels() => (_lit.Count, _defaultLit);

    public Image Next()
    {
        var next = new Image(_iea)
        {
            _defaultLit = _defaultLit
                ? _iea.Contains(LightRegion)
                : _iea.Contains(DarkRegion),
        };

        foreach (var (row, col) in _bounds.Stretched(1).IterRowCol())
        {
            var idx = ComputeIeaIndex(row, col);
            if (_iea.Contains(idx))
            {
                next._lit.Add((row, col));
                next._bounds.UpdateWith(row, col);
            }
        }

        return next;
    }

    private ushort ComputeIeaIndex(long row, long col)
    {
        ushort idx = 0;
        ushort mask = 0b100_000_000;

        for (var y = row - 1; y <= row + 1; y += 1)
        {
            for (var x = col - 1; x <= col + 1; x += 1)
            {
                if (IsLit(y, x))
                    idx |= mask;
                mask >>= 1;
            }
        }

        return idx;
    }

    private bool IsLit(long row, long col) =>
        _defaultLit && !_bounds.Contains(row, col) || _lit.Contains((row, col));

    public static Image Parse(IEnumerable<string> lines)
    {
        using var iter = lines.GetEnumerator();

        // first line contains the image enhancement algorithm
        if (!iter.MoveNext())
            throw new FormatException("Invalid input");
        var image = new Image(ParseIea(iter.Current));

        // second line must be empty
        if (!iter.MoveNext() || !string.IsNullOrEmpty(iter.Current))
            throw new FormatException("Invalid input");

        // rest of the lines describe the pixels
        var row = 0;
        while (iter.MoveNext())
        {
            ParsePixels(iter.Current, row, image);
            row += 1;
        }

        return image;
    }

    private static HashSet<ushort> ParseIea(string line)
    {
        var iea = new HashSet<ushort>();
        for (ushort idx = 0; idx < line.Length; idx += 1)
        {
            switch (line[idx])
            {
                case '#':
                    iea.Add(idx);
                    break;
                case '.':
                    break;
                default:
                    throw new FormatException($"Invalid character {line[idx]} @ {idx} : {line}");
            }
        }
        return iea;
    }

    private static void ParsePixels(string line, long row, Image image)
    {
        for (var col = 0; col < line.Length; col += 1)
        {
            switch (line[col])
            {
                case '#':
                    image._lit.Add((row, col));
                    image._bounds.UpdateWith(row, col);
                    break;
                case '.':
                    break;
                default:
                    throw new FormatException($"Invalid character {line[col]} @ {col} : {line}");
            }
        }
    }
}
