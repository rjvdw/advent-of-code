namespace Day09;

public static class Solution
{
    public static uint CalculateRisk(IEnumerable<byte> values) =>
        values.Select(v => 1u + v).Aggregate((a, v) => a + v);
}
