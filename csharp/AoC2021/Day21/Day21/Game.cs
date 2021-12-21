namespace Day21;

public class Game
{
    private readonly Player[] _players;
    private int _turn;
    private readonly int _targetScore;

    public Game(IReadOnlyList<Player> players, int targetScore)
    {
        _players = new[] { players[0], players[1] };
        _turn = 0;
        _targetScore = targetScore;
    }

    public (int Turn, Player[] Players)? Play(int roll)
    {
        var player = _players[_turn].Move(roll);
        _players[_turn] = player;

        if (player.HasWon(_targetScore))
            return (_turn, new[] { _players[0], _players[1] });

        _turn ^= 1;
        return null;
    }
}
