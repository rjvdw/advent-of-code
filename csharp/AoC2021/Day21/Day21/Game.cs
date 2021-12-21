namespace Day21;

public class Game : IEquatable<Game>
{
    private readonly Player[] _players;
    private readonly int _turn;
    private readonly int _targetScore;

    public Player[] Players => new[] { _players[0], _players[1] };

    public Game(IReadOnlyList<Player> players, int targetScore)
    {
        _players = new[] { players[0], players[1] };
        _turn = 0;
        _targetScore = targetScore;
    }

    private Game(Player[] players, int turn, int targetScore)
    {
        _players = players;
        _turn = turn;
        _targetScore = targetScore;
    }

    public (Game Game, int? Turn) Play(int roll)
    {
        var nextPlayers = new[] { _players[0], _players[1] };
        var player = nextPlayers[_turn] = _players[_turn].Move(roll);
        var nextGame = new Game(nextPlayers, _turn ^ 1, _targetScore);

        return player.HasWon(_targetScore)
            ? (nextGame, _turn)
            : (nextGame, null);
    }

    public bool Equals(Game? other) =>
        other is not null &&
        _players[0].Equals(other._players[0]) &&
        _players[1].Equals(other._players[1]) &&
        _turn == other._turn &&
        _targetScore == other._targetScore;

    public override bool Equals(object? obj) =>
        !ReferenceEquals(null, obj) &&
        (ReferenceEquals(this, obj) || obj.GetType() == GetType() && Equals((Game)obj));

    public override int GetHashCode() => HashCode.Combine(_players[0], _players[1], _turn, _targetScore);
}
