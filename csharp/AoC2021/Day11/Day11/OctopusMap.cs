using Shared;

namespace Day11;

public record OctopusMap : DigitMap
{
    public (OctopusMap Next, int NrFlashes) Tick()
    {
        var next = new List<byte>(new byte[Count]);
        var goingToFlash = new Stack<int>();
        var flashed = new HashSet<int>();

        for (var i = 0; i < next.Count; i += 1)
        {
            next[i] = (byte)(this[i] + 1);
            if (next[i] > 9)
                goingToFlash.Push(i);
        }

        while (goingToFlash.Count > 0)
        {
            var i = goingToFlash.Pop();
            if (!flashed.Contains(i))
            {
                flashed.Add(i);
                foreach (var (row, col) in Neighbours(GetRowAndCol(i), true))
                {
                    var j = GetIndex(row, col);
                    next[j] = (byte)(next[j] + 1);
                    if (next[j] > 9)
                        goingToFlash.Push(j);
                }
            }
        }

        foreach (var i in flashed)
            next[i] = 0;

        return (new OctopusMap(next, Rows, Cols), flashed.Count);
    }

    public override string ToString() => base.ToString();

    private OctopusMap(List<byte> values, int rows, int cols) : base(values, rows, cols)
    {
    }

    public static OctopusMap Parse(IEnumerable<string> lines)
    {
        var (values, rows, cols) = ParseLines(lines);
        return new OctopusMap(values, rows, cols);
    }
}
