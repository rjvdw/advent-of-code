namespace Day18;

public static class Solution
{
    public static long DoHomeWork1(IEnumerable<SnailNumber> snailNumbers) =>
        snailNumbers.Aggregate((a, b) => a + b).Magnitude();

    public static long DoHomeWork2(List<SnailNumber> snailNumbers)
    {
        var max = long.MinValue;

        for (var i = 0; i < snailNumbers.Count; i += 1)
        {
            for (var j = 0; j < snailNumbers.Count; j += 1)
            {
                if (i != j)
                {
                    var m = (snailNumbers[i] + snailNumbers[j]).Magnitude();
                    if (m > max)
                        max = m;
                }
            }
        }

        return max;
    }
}
