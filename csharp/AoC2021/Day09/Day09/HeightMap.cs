using Shared;

namespace Day09;

public record HeightMap : DigitMap
{
    public IEnumerable<(int Row, int Col)> FindLowPoints()
    {
        var lowPoints = new List<(int, int)>();
        for (var row = 0; row < Rows; row += 1)
        {
            for (var col = 0; col < Cols; col += 1)
            {
                var value = this[(row, col)];
                var isLowPoint = Neighbours((row, col), false)
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
                    from neighbour in Neighbours(toExplore.Pop(), false)
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

    public override string ToString() => base.ToString();

    private HeightMap(List<byte> heights, int rows, int cols) : base(heights, rows, cols)
    {
    }

    public static HeightMap Parse(IEnumerable<string> lines)
    {
        var (heights, rows, cols) = ParseLines(lines);
        return new HeightMap(heights, rows, cols);
    }
}
