using System.Text;

namespace Shared;

public abstract record DigitMap(List<byte> _values, int Rows, int Cols)
{
    private readonly List<byte> _values = _values;

    /// <summary>
    /// The number of rows in this map.
    /// </summary>
    public readonly int Rows = Rows;

    /// <summary>
    /// The number of columns in this map.
    /// </summary>
    public readonly int Cols = Cols;

    /// <summary>
    /// The total number of elements in this map.
    /// </summary>
    public int Count => Rows * Cols;

    /// <summary>
    /// Get the value at the specified <paramref name="index"/>.
    /// </summary>
    /// <param name="index">The <code>Row</code> and <code>Col</code> from where to read the value.</param>
    public byte this[(int Row, int Col) index] => _values[GetIndex(index.Row, index.Col)];

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
    /// Get the value at the specified index.
    /// </summary>
    /// <param name="i">The index at which to get the value.</param>
    protected byte this[int i] => _values[i];

    /// <summary>
    /// Computes the index in the values list for a given position.
    /// </summary>
    /// <param name="row">The row index.</param>
    /// <param name="col">The column index.</param>
    /// <returns>The list index.</returns>
    protected int GetIndex(int row, int col) => row * Cols + col;

    /// <summary>
    /// Computes the row and column for a given index.
    /// </summary>
    /// <param name="i">The list index.</param>
    /// <returns>The row and column indices.</returns>
    protected (int Row, int Col) GetRowAndCol(int i) => (i / Cols, i % Cols);

    /// <summary>
    /// Determines all neighbours for a given position.
    /// </summary>
    /// <param name="position">The position for which to return the neighbours.</param>
    /// <param name="includeDiagonals">Whether or not to also consider neighbours on diagonals.</param>
    /// <returns>All neighbours for the given position.</returns>
    protected IEnumerable<(int Row, int Col)> Neighbours((int Row, int Col) position, bool includeDiagonals)
    {
        var neighbours = new List<(int Row, int Col)>();

        var atTopEdge = position.Row <= 0;
        var atLeftEdge = position.Col <= 0;
        var atBottomEdge = position.Row + 1 >= Rows;
        var atRightEdge = position.Col + 1 >= Cols;

        if (!atTopEdge) neighbours.Add((position.Row - 1, position.Col));
        if (!atLeftEdge) neighbours.Add((position.Row, position.Col - 1));
        if (!atBottomEdge) neighbours.Add((position.Row + 1, position.Col));
        if (!atRightEdge) neighbours.Add((position.Row, position.Col + 1));

        if (includeDiagonals)
        {
            if (!atTopEdge && !atLeftEdge) neighbours.Add((position.Row - 1, position.Col - 1));
            if (!atTopEdge && !atRightEdge) neighbours.Add((position.Row - 1, position.Col + 1));
            if (!atBottomEdge && !atLeftEdge) neighbours.Add((position.Row + 1, position.Col - 1));
            if (!atBottomEdge && !atRightEdge) neighbours.Add((position.Row + 1, position.Col + 1));
        }

        return neighbours;
    }

    /// <summary>
    /// Helper method to parse lines into a <see cref="DigitMap"/>.
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
