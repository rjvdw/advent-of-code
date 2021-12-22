namespace Day20;

public class Image
{
    private const ushort DarkRegion = 0b000_000_000;
    private const ushort LightRegion = 0b111_111_111;

    private readonly bool[] _iea;
    private readonly bool[][] _lit;
    private readonly int _height;
    private readonly int _width;
    private readonly bool _areOutOfBoundsPixelsLit;

    public Image Next()
    {
        var newHeight = _height + 2;
        var newWidth = _width + 2;

        var lit = Enumerable.Range(0, newHeight)
            .Select(row =>
                Enumerable.Range(0, newWidth)
                    .Select(col => GetIeaIndex(row, col))
                    .Select(idx => _iea[idx])
                    .ToArray()
            ).ToArray();

        var areOutOfBoundsPixelsLit = _areOutOfBoundsPixelsLit
            ? _iea[LightRegion]
            : _iea[DarkRegion];

        return new Image(
            _iea,
            lit,
            newHeight,
            newWidth,
            areOutOfBoundsPixelsLit
        );
    }

    public (int Count, bool IsInfinite) CountLitPixels() =>
        (_lit.Select(x => x.Count(v => v)).Sum(), _areOutOfBoundsPixelsLit);

    private int GetIeaIndex(int row, int col)
    {
        var index = 0;
        var mask = 0b100_000_000;

        for (var y = row; y <= row + 2; y += 1)
        {
            for (var x = col; x <= col + 2; x += 1)
            {
                var isOnEdge = y < 2 || x < 2;
                var isLit = isOnEdge
                    ? _areOutOfBoundsPixelsLit
                    : IsLit(y - 2, x - 2);

                if (isLit)
                    index |= mask;

                mask >>= 1;
            }
        }

        return index;
    }

    private bool IsLit(int row, int col) =>
        row < _height && col < _width
            ? _lit[row][col]
            : _areOutOfBoundsPixelsLit;

    private Image(bool[] iea, bool[][] lit, int height, int width, bool areOutOfBoundsPixelsLit)
    {
        _iea = iea;
        _lit = lit;
        _height = height;
        _width = width;
        _areOutOfBoundsPixelsLit = areOutOfBoundsPixelsLit;
    }

    public static Image Parse(IEnumerable<string> lines)
    {
        using var iter = lines.GetEnumerator();

        // first line contains the image enhancement algorithm
        if (!iter.MoveNext())
            throw new FormatException("Invalid input");
        var iea = ParseLine(iter.Current);

        // second line must be empty
        if (!iter.MoveNext() || !string.IsNullOrEmpty(iter.Current))
            throw new FormatException("Invalid input");

        // rest of the lines describe the pixels
        var lit = new List<bool[]>();
        var height = 0;
        var width = 0;
        while (iter.MoveNext())
        {
            height += 1;
            width = iter.Current.Length;
            lit.Add(ParseLine(iter.Current));
        }

        return new Image(
            iea,
            lit.ToArray(),
            height,
            width,
            false
        );
    }

    private static bool[] ParseLine(string line) => line.Select(ch => ch == '#').ToArray();
}
