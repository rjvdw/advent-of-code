namespace Day03;

// https://adventofcode.com/2020/day/3
public static class Program
{
    public static void Main(string[] args)
    {
        if (args.Length != 1)
        {
            Console.Error.WriteLine("Usage: $0 <input file>");
            Environment.Exit(1);
            return;
        }

        var inputFile = args[0];
        var track = new Track(File.ReadAllLines(inputFile));

        Console.WriteLine($"The solution to part 1 is: {track.CountTrees(3, 1)}");

        var solution2 = (long)track.CountTrees(1, 1) *
                        track.CountTrees(3, 1) *
                        track.CountTrees(5, 1) *
                        track.CountTrees(7, 1) *
                        track.CountTrees(1, 2);

        Console.WriteLine($"The solution to part 2 is: {solution2}");
    }
}

public class Track
{
    private readonly bool[][] _rows;
    private readonly int _width;
    private readonly int _height;

    public Track(IEnumerable<string> lines)
    {
        _rows = lines
            .Select(row => row
                .Select(ch => ch == '#')
                .ToArray())
            .ToArray();
        _width = _rows[0].Length;
        _height = _rows.Length;
    }

    public int CountTrees(int right, int down)
    {
        var nrTrees = 0;
        for (var position = new Point(0, 0); position.Y < _height; position += (right, down))
        {
            if (IsTreeAt(position))
                nrTrees += 1;
        }

        return nrTrees;
    }

    private bool IsTreeAt(Point point) => _rows[point.Y][point.X % _width];

    public override string ToString() =>
        string.Join('\n', _rows
            .Select(row => string.Join("", row
                .Select(cell => cell ? '#' : '.'))));
}

internal record Point(int X, int Y)
{
    public static implicit operator Point((int x, int y) point) => new(point.x, point.y);

    public static Point operator +(Point p1, Point p2) => new(p1.X + p2.X, p1.Y + p2.Y);
}
