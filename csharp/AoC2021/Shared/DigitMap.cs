using System.Text;

namespace Shared;

public abstract record DigitMap(List<byte> _values, int Rows, int Cols)
{
    private readonly List<byte> _values = _values;
    protected readonly int Rows = Rows;
    protected readonly int Cols = Cols;

    /// <summary>
    /// Get the value at the specified <code>Row</code> and <code>Col</code>.
    /// </summary>
    /// <param name="index">The <code>Row</code> and <code>Col</code> from where to read the value.</param>
    public byte this[(int Row, int Col) index] => _values[index.Row * Cols + index.Col];

    public override string ToString()
    {
        StringBuilder sb = new();
        for (var i = 0; i < _values.Count; i += 1)
        {
            if (i != 0 && i % Cols == 0)
                sb.Append('\n');
            sb.Append(_values[i]);
        }

        return sb.ToString();
    }

    /// <summary>
    /// Determines all neighbours for a given position.
    /// </summary>
    /// <param name="position">The position for which to return the neighbours.</param>
    /// <param name="includeDiagonals">Whether or not to also consider neighbours on diagonals.</param>
    /// <returns>All neighbours for the given position.</returns>
    protected IEnumerable<(int Row, int Col)> Neighbours((int Row, int Col) position, bool includeDiagonals)
    {
        var neighbours = new List<(int Row, int Col)>();
        if (position.Row > 0) neighbours.Add((position.Row - 1, position.Col));
        if (position.Col > 0) neighbours.Add((position.Row, position.Col - 1));
        if (position.Row + 1 < Rows) neighbours.Add((position.Row + 1, position.Col));
        if (position.Col + 1 < Cols) neighbours.Add((position.Row, position.Col + 1));
        if (includeDiagonals)
        {
            // TODO
        }
        return neighbours;
    }

    /// <summary>
    /// Helper method to parse lines into a <code>DigitMap</code>.
    /// </summary>
    /// <param name="lines">The lines to parse.</param>
    /// <returns>The values, row count, and column count that were determined from the input lines.</returns>
    protected static (List<byte> Values, int Rows, int Cols) ParseLines(IEnumerable<string> lines)
    {
        var values = new List<byte>();
        var rows = 0;
        var cols = 0;

        foreach (var line in lines)
        {
            rows += 1;
            cols = line.Length;
            values.AddRange(line.ToCharArray().Select(ch => (byte)(ch - '0')));
        }

        return (values, rows, cols);
    }
}
