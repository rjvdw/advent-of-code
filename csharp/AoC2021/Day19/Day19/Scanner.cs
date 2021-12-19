using System.Text;

namespace Day19;

public class Scanner
{
    private const string ScannerHeaderPrefix = "--- scanner ";
    private const string ScannerHeaderSuffix = " ---";

    public readonly int Idx;
    public readonly ushort Orientation;
    public readonly Point Position;
    public readonly HashSet<Point> Beacons;
    private readonly List<long> _distances;

    private Scanner(int idx, ushort orientation, Point position, HashSet<Point> beacons, List<long> distances)
    {
        Idx = idx;
        Orientation = orientation;
        Position = position;
        Beacons = beacons;
        _distances = distances;
    }

    public static IEnumerable<Scanner> Parse(IEnumerable<string> lines)
    {
        var idx = 0;
        var orientation = (ushort)0;
        var position = new Point(0, 0, 0);
        var beacons = new HashSet<Point>();
        var distances = new List<long>();

        foreach (var line in lines)
        {
            if (string.IsNullOrEmpty(line))
            {
                yield return new Scanner(idx, orientation, position, beacons, distances);
                idx = 0;
                orientation = 0;
                position = new Point(0, 0, 0);
                beacons = new HashSet<Point>();
                distances = new List<long>();
            }
            else if (line.StartsWith(ScannerHeaderPrefix))
            {
                var i = line.IndexOf(ScannerHeaderSuffix, ScannerHeaderPrefix.Length, StringComparison.Ordinal);
                if (i == -1)
                    throw new FormatException($"Invalid input line: {line}");
                idx = int.Parse(line[ScannerHeaderPrefix.Length..i]);
            }
            else
            {
                var beacon = Point.Parse(line);
                distances.AddRange(beacons.Select(beacon.DistanceTo));
                distances.Sort();
                beacons.Add(beacon);
            }
        }

        yield return new Scanner(idx, orientation, position, beacons, distances);
    }
}
