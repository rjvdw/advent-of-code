using System.Text;

namespace Day13;

public record Manual
{
    private HashSet<(int X, int Y)> _dots;
    private readonly List<(int? X, int? Y)> _folds;
    private int _pointer;

    public int VisibleDots => _dots.Count;
    public int FoldsRemaining => _folds.Count - _pointer;

    public void Fold()
    {
        var fold = _folds[_pointer];
        _pointer += 1;
        var folded = new HashSet<(int X, int Y)>();

        if (fold.X.HasValue)
        {
            var x = fold.X.Value;
            foreach (var dot in _dots)
                folded.Add(dot.X > x
                    ? (x - (dot.X - x), dot.Y)
                    : dot);
        }
        else // either x or y must have a value
        {
            var y = fold.Y!.Value;
            foreach (var dot in _dots)
                folded.Add(dot.Y > y
                    ? (dot.X, y - (dot.Y - y))
                    : dot);
        }

        _dots = folded;
    }

    public override string ToString()
    {
        var minX = int.MaxValue;
        var maxX = int.MinValue;
        var minY = int.MaxValue;
        var maxY = int.MinValue;

        foreach (var (x, y) in _dots)
        {
            if (x < minX) minX = x;
            if (x > maxX) maxX = x;
            if (y < minY) minY = y;
            if (y > maxY) maxY = y;
        }

        var sb = new StringBuilder();

        for (var y = minY; y <= maxY; y += 1)
        {
            if (y != minY) sb.AppendLine();
            for (var x = minX; x <= maxX; x += 1)
                sb.Append(_dots.Contains((x, y)) ? '#' : '.');
        }

        return sb.ToString();
    }

    private Manual(HashSet<(int x, int y)> dots, List<(int? X, int? Y)> folds)
    {
        _dots = dots;
        _folds = folds;
    }

    public static Manual Parse(IEnumerable<string> lines)
    {
        var dots = new HashSet<(int X, int Y)>();
        var folds = new List<(int? X, int? Y)>();

        foreach (var line in lines)
        {
            var iComma = line.IndexOf(',');
            var iFoldX = line.IndexOf("x=", StringComparison.Ordinal);
            var iFoldY = line.IndexOf("y=", StringComparison.Ordinal);

            if (iComma != -1)
            {
                var x = int.Parse(line[..iComma]);
                var y = int.Parse(line[(iComma + 1)..]);
                dots.Add((x, y));
            }
            else if (iFoldX != -1)
                folds.Add((int.Parse(line[(iFoldX + 2)..]), null));
            else if (iFoldY != -1)
                folds.Add((null, int.Parse(line[(iFoldY + 2)..])));
            else if (!string.IsNullOrEmpty(line))
                throw new ArgumentException($"Invalid input ('{line}')", nameof(lines));
        }

        return new Manual(dots, folds);
    }
}
