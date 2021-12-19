namespace Day19;

public class Scanner
{
    private const string ScannerHeaderPrefix = "--- scanner ";
    private const string ScannerHeaderSuffix = " ---";

    private const int Threshold = 12;
    private const int DistanceThreshold = Threshold * (Threshold - 1) / 2;

    private readonly int _idx;
    public ushort Orientation { get; }
    public Point Position { get;  }
    private readonly HashSet<Point> _beacons;
    private readonly List<long> _distances;

    public IEnumerable<Point> Beacons => _beacons;
    public IEnumerable<long> Distances => _distances;

    public Scanner? Adjust(Scanner other)
    {
        if (CountDistanceOverlap(other) < DistanceThreshold)
        {
            // there is no way these scanners can have overlap, because the distances are too different
            return null;
        }

        foreach (var sBeacon in _beacons)
        {
            foreach (var oBeacon in other._beacons)
            {
                for (ushort orientation = 0; orientation < Point.Orientations; orientation += 1)
                {
                    var offset = sBeacon - oBeacon.Rotate(orientation);
                    var scanner = other.Transform(offset, orientation);
                    if (CountOverlap(scanner) >= Threshold)
                        return scanner;
                }
            }
        }

        return null;
    }

    private Scanner Transform(Point offset, ushort orientation) =>
        new(
            _idx,
            orientation,
            offset,
            _beacons
                .Select(p => p.Rotate(orientation))
                .Select(p => p + offset)
                .ToHashSet(),
            _distances
        );

    private int CountOverlap(Scanner other) => _beacons.Intersect(other._beacons).Count();

    private int CountDistanceOverlap(Scanner other)
    {
        var count = 0;
        var i = 0;
        var j = 0;

        while (i < _distances.Count && j < other._distances.Count)
        {
            if (_distances[i] < other._distances[j])
            {
                i += 1;
            }
            else if (_distances[i] == other._distances[j])
            {
                count += 1;
                i += 1;
                j += 1;
            }
            else
            {
                j += 1;
            }
        }

        return count;
    }

    private Scanner(int idx, ushort orientation, Point position, HashSet<Point> beacons, List<long> distances)
    {
        _idx = idx;
        Orientation = orientation;
        Position = position;
        _beacons = beacons;
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
