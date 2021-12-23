namespace Day23;

public static class CandidateExtensions
{
    public static bool ContainsNode(this IEnumerable<(Node Node, Amphipod Amphipod)> amphipods, Node node) =>
        amphipods.Any(v => v.Node == node);

    public static Amphipod? GetNode(this IEnumerable<(Node Node, Amphipod Amphipod)> amphipods, Node node) =>
        amphipods.Where(v => v.Node == node).Select(v => v.Amphipod).FirstOrDefault();
}
