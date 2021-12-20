namespace Day20;

public class Bounds
{
    public (long Row, long Col) TopLeft;
    public (long Row, long Col) BottomRight;

    public Bounds((long Row, long Col) topLeft, (long Row, long Col) bottomRight)
    {
        TopLeft = topLeft;
        BottomRight = bottomRight;
    }

    public void UpdateWith(long row, long col)
    {
        if (row < TopLeft.Row) TopLeft.Row = row;
        if (row > BottomRight.Row) BottomRight.Row = row;
        if (col < TopLeft.Col) TopLeft.Col = col;
        if (col > BottomRight.Col) BottomRight.Col = col;
    }

    public Bounds Stretched(long by) => new(
        (TopLeft.Row - by, TopLeft.Col - by),
        (BottomRight.Row + by, BottomRight.Col + by)
    );

    public IEnumerable<(long Row, long Col)> IterRowCol()
    {
        for (var row = TopLeft.Row; row <= BottomRight.Row; row += 1)
        {
            for (var col = TopLeft.Col; col <= BottomRight.Col; col += 1)
            {
                yield return (row, col);
            }
        }
    }

    public bool Contains(long row, long col) =>
        row >= TopLeft.Row && row <= BottomRight.Row &&
        col >= TopLeft.Col && col <= BottomRight.Col;
}
