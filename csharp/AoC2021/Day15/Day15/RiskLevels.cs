using System.Text;

namespace Day15;

public class RiskLevels
{
    private readonly List<int> _map;
    private readonly int _cols;
    private readonly int _rows;

    public int? FindShortestPath()
    {
        const int start = 0;
        var end = _map.Count - 1;

        var openSet = new HashSet<int>();
        var openSetQueue = new PriorityQueue<int, int>();
        openSet.Add(start);
        openSetQueue.Enqueue(start, HeuristicDistance(start, end));

        var cameFrom = new Dictionary<int, int>();
        var gScore = new Dictionary<int, int> { [start] = 0 };

        while (openSetQueue.TryDequeue(out var current, out _))
        {
            openSet.Remove(current);
            if (current == end)
            {
                var point = current;
                var score = 0;
                while (cameFrom.TryGetValue(point, out var p))
                {
                    score += _map[point];
                    point = p;
                }

                return score;
            }

            foreach (var (d, neighbour) in Neighbours(current))
            {
                var distance = gScore[current] + d;
                if (!gScore.ContainsKey(neighbour) || distance < gScore[neighbour])
                {
                    cameFrom[neighbour] = current;
                    gScore[neighbour] = distance;
                    if (!openSet.Contains(neighbour))
                    {
                        openSet.Add(neighbour);
                        openSetQueue.Enqueue(neighbour, distance + HeuristicDistance(neighbour, end));
                    }
                }
            }
        }

        return null;
    }

    public RiskLevels Transform()
    {
        var cols = _cols * 5;
        var rows = _rows * 5;
        var newSize = cols * rows;
        var map = new List<int>(newSize);
        for (var i = 0; i < newSize; i += 1)
        {
            var row = i / cols;
            var col = i % cols;
            var oRow = row % _rows;
            var oCol = col % _cols;
            map.Add((_map[oRow * _cols + oCol] + row / _rows + col / _cols - 1) % 9 + 1);
        }

        return new RiskLevels(map, cols, rows);
    }

    private int HeuristicDistance(int from, int to)
    {
        var (fromRow, fromCol) = Index(from);
        var (toRow, toCol) = Index(to);

        return Math.Abs(fromRow - toRow) + Math.Abs(fromCol - toCol);
    }

    private IEnumerable<(int Distance, int Neighbour)> Neighbours(int i)
    {
        var (row, col) = Index(i);
        return new[] { (row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1) }
            .Where(nb =>
            {
                var (r, c) = nb;
                return r >= 0 && c >= 0 && r < _rows && c < _cols;
            })
            .Select(nb =>
            {
                var (r, c) = nb;
                return r * _cols + c;
            })
            .Select(nb => (_map[nb], nb));
    }

    private (int, int) Index(int i) => (i / _cols, i % _cols);

    public override string ToString()
    {
        var sb = new StringBuilder();
        for (var i = 0; i < _map.Count; i += 1)
        {
            if (i != 0 && i % _cols == 0)
                sb.AppendLine();
            sb.Append(_map[i]);
        }

        return sb.ToString();
    }

    private RiskLevels(List<int> map, int cols, int rows)
    {
        if (map.Count != cols * rows)
            throw new ArgumentException("Input list has invalid number of elements.", nameof(map));
        _map = map;
        _cols = cols;
        _rows = rows;
    }

    public static RiskLevels Parse(IEnumerable<string> lines)
    {
        var map = new List<int>();
        var cols = 0;
        var rows = 0;

        foreach (var line in lines)
        {
            rows += 1;
            cols = line.Length;
            map.AddRange(line.Select(ch => ch - '0'));
        }

        return new RiskLevels(map, cols, rows);
    }
}
