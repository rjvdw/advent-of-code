using System.Text;

namespace Day12;

public class CaveMap
{
    private readonly HashSet<string> _nodes;
    private readonly Dictionary<string, HashSet<string>> _edges;

    private CaveMap(HashSet<string> nodes, Dictionary<string, HashSet<string>> edges)
    {
        _nodes = nodes;
        _edges = edges;
    }

    public int CountPaths(int maxRevisits)
    {
        var count = 0;
        var toExplore = new Stack<(string Current, HashSet<string> Seen, int RevisitsLeft)>();
        toExplore.Push(("start", new HashSet<string>(), maxRevisits));

        while (toExplore.Count > 0)
        {
            var (current, seen, revisitsLeft) = toExplore.Pop();
            foreach (var to in _edges[current])
            {
                if (to == "end")
                    count += 1;
                else if (revisitsLeft > 0 || !seen.Contains(to))
                {
                    var nextSeen = new HashSet<string>(seen);
                    if (to.ToLower() == to)
                        nextSeen.Add(to);

                    var nextRevisitsLeft = seen.Contains(to)
                        ? revisitsLeft - 1
                        : revisitsLeft;

                    toExplore.Push((to, nextSeen, nextRevisitsLeft));
                }
            }
        }

        return count;
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        sb.Append("CaveMap {\n");

        sb.Append("  nodes: { ");
        sb.Append(string.Join(", ", _nodes));
        sb.Append(" }\n");

        sb.Append("  edges: {\n");
        foreach (var (from, to) in _edges)
        {
            sb.Append("    ");
            sb.Append(from);
            sb.Append(": { ");
            sb.Append(string.Join(", ", to));
            sb.Append(" },\n");
        }

        sb.Append("  }");

        sb.Append('}');
        return sb.ToString();
    }

    public static CaveMap Parse(IEnumerable<string> lines)
    {
        var nodes = new HashSet<string>();
        var edges = new Dictionary<string, HashSet<string>>();

        foreach (var line in lines)
        {
            var p = line.IndexOf('-');
            if (p == -1)
                throw new ArgumentException("Invalid input provided.", nameof(lines));

            var n1 = line[..p];
            var n2 = line[(p + 1)..];

            nodes.Add(n1);
            nodes.Add(n2);

            if (n1 != "end" && n2 != "start")
            {
                if (!edges.ContainsKey(n1))
                    edges.Add(n1, new HashSet<string>());
                edges[n1].Add(n2);
            }

            if (n1 != "start" && n2 != "end")
            {
                if (!edges.ContainsKey(n2))
                    edges.Add(n2, new HashSet<string>());
                edges[n2].Add(n1);
            }
        }

        return new CaveMap(nodes, edges);
    }
}