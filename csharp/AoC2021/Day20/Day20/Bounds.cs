namespace Day20;

public class Bounds
{
    private (long Row, long Col) _topLeft;
    private (long Row, long Col) _bottomRight;

    public (long Row, long Col) TopLeft => _topLeft;
    public (long Row, long Col) BottomRight => _bottomRight;

    public Bounds((long Row, long Col) topLeft, (long Row, long Col) bottomRight)
    {
        _topLeft = topLeft;
        _bottomRight = bottomRight;
    }

    public void UpdateWith(long row, long col)
    {
        if (row < _topLeft.Row) _topLeft.Row = row;
        if (row > _bottomRight.Row) _bottomRight.Row = row;
        if (col < _topLeft.Col) _topLeft.Col = col;
        if (col > _bottomRight.Col) _bottomRight.Col = col;
    }

    public Bounds Stretched(long by) => new(
        (_topLeft.Row - by, _topLeft.Col - by),
        (_bottomRight.Row + by, _bottomRight.Col + by)
    );

    public IEnumerable<(long Row, long Col)> IterRowCol()
    {
        for (var row = _topLeft.Row; row <= _bottomRight.Row; row += 1)
        {
            for (var col = _topLeft.Col; col <= _bottomRight.Col; col += 1)
            {
                yield return (row, col);
            }
        }
    }

    public bool Contains(long row, long col) =>
        row >= _topLeft.Row && row <= _bottomRight.Row &&
        col >= _topLeft.Col && col <= _bottomRight.Col;
}
