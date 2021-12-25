namespace Day25;

public class SeaFloor
{
    private readonly List<SeaCucumber> _seaCucumbers;
    private readonly int _rows;
    private readonly int _cols;

    private SeaFloor Empty() => new(
        Enumerable.Repeat(SeaCucumber.Empty, _rows * _cols).ToList(),
        _rows,
        _cols
    );

    public bool TryNext(out SeaFloor next)
    {
        next = Empty();
        var anyoneMoved = false;

        for (var i = 0; i < _seaCucumbers.Count; i += 1)
        {
            switch (_seaCucumbers[i])
            {
                case SeaCucumber.East:
                {
                    var destination = GetRowOffset(i) + GetCol(i + 1);
                    if (_seaCucumbers[destination].IsEmpty())
                    {
                        next._seaCucumbers[destination] = SeaCucumber.East;
                        anyoneMoved = true;
                    }
                    else
                    {
                        next._seaCucumbers[i] = SeaCucumber.East;
                    }

                    break;
                }
                case SeaCucumber.South:
                {
                    var destination = (i + _cols) % (_rows * _cols);
                    var left = GetRowOffset(destination) + GetCol(destination - 1);
                    var right = GetRowOffset(destination) + GetCol(destination + 1);

                    var isFree = _seaCucumbers[destination].IsEmpty() && !_seaCucumbers[left].IsEast();
                    var willBeFree = _seaCucumbers[destination].IsEast() && _seaCucumbers[right].IsEmpty();

                    if (isFree || willBeFree)
                    {
                        next._seaCucumbers[destination] = SeaCucumber.South;
                        anyoneMoved = true;
                    }
                    else
                    {
                        next._seaCucumbers[i] = SeaCucumber.South;
                    }

                    break;
                }
            }
        }

        return anyoneMoved;
    }

    private int GetRowOffset(int i) => i - (i % _cols);

    private int GetCol(int i) => (i < 0 ? i + _rows * _cols : i) % _cols;

    private SeaFloor(List<SeaCucumber> seaCucumbers, int rows, int cols)
    {
        _seaCucumbers = seaCucumbers;
        _rows = rows;
        _cols = cols;
    }

    public static SeaFloor Parse(IEnumerable<string> input)
    {
        var cucumbers = new List<SeaCucumber>();

        var rows = 0;
        var cols = 0;
        foreach (var line in input)
        {
            rows += 1;
            cols = 0;
            foreach (var ch in line)
            {
                cols += 1;
                cucumbers.Add(ch switch
                {
                    '>' => SeaCucumber.East,
                    'v' => SeaCucumber.South,
                    _ => SeaCucumber.Empty,
                });
            }
        }

        return new SeaFloor(cucumbers, rows, cols);
    }
}
