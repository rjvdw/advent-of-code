using System.Text;

namespace Day12;

public class CaveMap
{
    private readonly Dictionary<string, HashSet<string>> _edges;

    private CaveMap(Dictionary<string, HashSet<string>> edges)
    {
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

    public static CaveMap Parse(IEnumerable<string> lines)
    {
        var edges = new Dictionary<string, HashSet<string>>();

        foreach (var line in lines)
        {
            var p = line.IndexOf('-');
            var n1 = line[..p];
            var n2 = line[(p + 1)..];

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

        return new CaveMap(edges);
    }
}
