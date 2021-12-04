namespace Day04;

public class Board
{
    private readonly List<int> _numbers;
    private readonly HashSet<int> _marked;
    private int _dim;

    public Board()
    {
        _numbers = new List<int>();
        _marked = new HashSet<int>();
        _dim = 0;
    }

    public Board(int dim, List<int> numbers)
    {
        if (numbers.Count != dim * dim)
            throw new ArgumentException("Invalid input length", nameof(numbers));
        _numbers = numbers;
        _marked = new HashSet<int>();
        _dim = dim;
    }

    public void AddRow(List<int> row)
    {
        if (_numbers.Count == 0)
            _dim = row.Count;
        else if (_dim != row.Count)
            throw new ArgumentException("Row has an invalid size", nameof(row));

        _numbers.AddRange(row);
    }

    public void Reset()
    {
        _marked.Clear();
    }

    public void Mark(int nr)
    {
        for (var i = 0; i < _numbers.Count; i += 1)
            if (_numbers[i] == nr)
                _marked.Add(i);
    }

    public bool Bingo()
    {
        for (var i = 0; i < _dim; i += 1)
        {
            var fullCol = true;
            var fullRow = true;

            for (var j = 0; j < _dim; j += 1)
            {
                if (!_marked.Contains(Index(i, j))) fullCol = false;
                if (!_marked.Contains(Index(j, i))) fullRow = false;
            }

            if (fullCol || fullRow) return true;
        }

        return false;
    }

    public int Score()
    {
        var score = 0;
        for (var i = 0; i < _numbers.Count; i += 1)
            if (!_marked.Contains(i))
                score += _numbers[i];
        return score;
    }

    private int Index(int row, int col) => row * _dim + col;
}
