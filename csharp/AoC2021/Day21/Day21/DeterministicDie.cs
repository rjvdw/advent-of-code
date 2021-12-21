namespace Day21;

public class DeterministicDie : IDie
{
    private readonly int _sides;
    private int _value;
    public int Rolls { get; private set; }

    public DeterministicDie(int sides)
    {
        _sides = sides;
        _value = 1;
        Rolls = 0;
    }

    public int Roll()
    {
        var value = _value;
        _value = _value == _sides ? 1 : value + 1;
        Rolls += 1;

        return value;
    }

    public int Roll(int times)
    {
        var value = 0;
        for (var i = 0; i < times; i += 1)
            value += Roll();
        return value;
    }
}
