namespace Day21;

public record Game
{
    private readonly Player _player1;
    private readonly Player _player2;
    private readonly int _turn;
    private readonly int _targetScore;

    public Player[] Players => new[] { _player1, _player2 };

    public Game(IReadOnlyList<Player> players, int targetScore)
    {
        _player1 = players[0];
        _player2 = players[1];
        _turn = 0;
        _targetScore = targetScore;
    }

    private Game(Player player1, Player player2, int turn, int targetScore)
    {
        _player1 = player1;
        _player2 = player2;
        _turn = turn;
        _targetScore = targetScore;
    }

    public (Game Game, int? Turn) Play(int roll)
    {
        var nextPlayers = Players;
        var player = nextPlayers[_turn] = nextPlayers[_turn].Move(roll);
        var nextGame = new Game(nextPlayers[0], nextPlayers[1], _turn ^ 1, _targetScore);

        return player.HasWon(_targetScore)
            ? (nextGame, _turn)
            : (nextGame, null);
    }
}
