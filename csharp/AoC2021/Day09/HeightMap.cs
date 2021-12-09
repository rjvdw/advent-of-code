using System.Text;

namespace Day09;

public record HeightMap
{
    private readonly List<byte> _heights;
    private readonly int _rows;
    private readonly int _cols;

    public byte this[(int Row, int Col) index] => _heights[index.Row * _cols + index.Col];

    public IEnumerable<(int Row, int Col)> FindLowPoints()
    {
        var lowPoints = new List<(int, int)>();
        for (var row = 0; row < _rows; row += 1)
        {
            for (var col = 0; col < _cols; col += 1)
            {
                var value = this[(row, col)];
                var isLowPoint = Neighbours((row, col))
                    .Select(p => this[p])
                    .All(v => v > value);
                if (isLowPoint)
                {
                    lowPoints.Add((row, col));
                }
            }
        }

        return lowPoints;
    }

    public IEnumerable<int> FindBasins(IEnumerable<(int Row, int Col)> points)
    {
        var sizes = new List<int>();
        var seen = new HashSet<(int Row, int Col)>();
        var toExplore = new Stack<(int Row, int Col)>();

        foreach (var point in points)
        {
            if (seen.Contains(point))
                throw new InvalidOperationException("There seem to be multiple low points in a single basin!");

            var count = 0;
            seen.Add(point);
            toExplore.Push(point);

            while (toExplore.Count != 0)
            {
                count += 1;

                var neighbours =
                    from neighbour in Neighbours(toExplore.Pop())
                    where this[neighbour] != 9 && !seen.Contains(neighbour)
                    select neighbour;

                foreach (var neighbour in neighbours)
                {
                    seen.Add(neighbour);
                    toExplore.Push(neighbour);
                }
            }

            sizes.Add(count);
        }

        return sizes;
    }

    public override string ToString()
    {
        StringBuilder sb = new();
        for (var i = 0; i < _heights.Count; i += 1)
        {
            if (i != 0 && i % _cols == 0)
                sb.Append('\n');
            sb.Append(_heights[i]);
        }

        return sb.ToString();
    }

    private HeightMap(List<byte> heights, int rows, int cols)
    {
        _heights = heights;
        _rows = rows;
        _cols = cols;
    }

    private IEnumerable<(int Row, int Col)> Neighbours((int Row, int Col) position)
    {
        var neighbours = new List<(int Row, int Col)>();
        if (position.Row > 0) neighbours.Add((position.Row - 1, position.Col));
        if (position.Col > 0) neighbours.Add((position.Row, position.Col - 1));
        if (position.Row + 1 < _rows) neighbours.Add((position.Row + 1, position.Col));
        if (position.Col + 1 < _cols) neighbours.Add((position.Row, position.Col + 1));
        return neighbours;
    }

    public static HeightMap Parse(IEnumerable<string> lines)
    {
        var heights = new List<byte>();
        var rows = 0;
        var cols = 0;

        foreach (var line in lines)
        {
            rows += 1;
            cols = line.Length;
            heights.AddRange(line.ToCharArray().Select(ch => (byte)(ch - '0')));
        }

        return new HeightMap(heights, rows, cols);
    }
}
