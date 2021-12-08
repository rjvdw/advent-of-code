module Board

open System

type Board =
    { Numbers: List<byte>
      Dim: int
      Marked: Set<int> }

let mark (number: byte) (board: Board) =
    match board.Numbers
          |> List.tryFindIndex (fun x -> x = number) with
    | Some idx ->
        { Numbers = board.Numbers
          Dim = board.Dim
          Marked = board.Marked |> Set.add idx }
    | _ -> board

let index (row: int, col: int) (board: Board) = row * board.Dim + col

let bingo (board: Board) =
    let indexes = seq { 0 .. board.Dim - 1 }

    let test idx x =
        indexes
        |> Seq.forall (fun y -> Set.contains (idx x y) board.Marked)

    let testRow =
        test (fun row col -> index (row, col) board)

    let testCol =
        test (fun col row -> index (row, col) board)

    indexes
    |> Seq.exists (fun x -> testRow x || testCol x)

let score (number: byte) (board: Board) =
    let sum =
        board.Numbers
        |> List.indexed
        |> List.filter (fun (idx, _) -> not (Set.contains idx board.Marked))
        |> List.map snd
        |> List.map uint32
        |> List.sum

    (uint32 number) * sum

let parseBoard (lines: seq<string>) =
    let dim, numbers =
        lines
        |> Seq.map (fun line -> line.Split(' ', StringSplitOptions.RemoveEmptyEntries))
        |> Seq.map List.ofSeq
        |> Seq.map (List.map byte)
        |> Seq.fold (fun (_, numbers) line -> (line.Length, List.append numbers line)) (0, [])

    { Numbers = numbers
      Dim = dim
      Marked = Set.empty }

let parseBoards (lines: seq<string>) =
    let folder (groups: list<list<string>>, nextGroup: list<string>) (line: string) =
        if line = "" then
            (nextGroup |> List.rev) :: groups, []
        else
            groups, line :: nextGroup

    let groups, nextGroup = lines |> Seq.fold folder ([ [] ], [])

    (nextGroup :: groups)
    |> List.rev
    |> List.map parseBoard

module tests =
    open Xunit

    [<Fact>]
    let ``Parsing of a squid bingo board should return a valid board`` () =
        let lines =
            [ "22 13 17 11  0"
              " 8  2 23  4 24"
              "21  9 14 16  7"
              " 6 10  3 18  5"
              " 1 12 20 15 19" ]

        let expected =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = Set.empty }

        Assert.Equal(expected, parseBoard lines)

    [<Fact>]
    let ``The correct position should be marked when marking a number`` () =
        let board =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = Set.empty }

        let board = mark 23uy board
        Assert.Equal(1, board.Marked.Count)
        Assert.True(board.Marked.Contains(7))

    [<Fact>]
    let ``A board without any numbers marked should not be marked as bingo`` () =
        let board =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = Set.empty }

        Assert.False(bingo board)

    [<Fact>]
    let ``A board with some numbers marked that do not form a row or column should not be marked as bingo`` () =
        let board =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = set [ 1; 2; 5; 8; 11; 16; 21 ] }

        Assert.False(bingo board)

    [<Fact>]
    let ``A board with a full column should be marked as bingo`` () =
        let board =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = set [ 1; 6; 11; 16; 21 ] }

        Assert.True(bingo board)

    [<Fact>]
    let ``A board with a full row should be marked as bingo`` () =
        let board =
            { Numbers =
                  [ 22uy
                    13uy
                    17uy
                    11uy
                    0uy
                    8uy
                    2uy
                    23uy
                    4uy
                    24uy
                    21uy
                    9uy
                    14uy
                    16uy
                    7uy
                    6uy
                    10uy
                    3uy
                    18uy
                    5uy
                    1uy
                    12uy
                    20uy
                    15uy
                    19uy ]
              Dim = 5
              Marked = set [ 5; 6; 7; 8; 9 ] }

        Assert.True(bingo board)

    [<Fact>]
    let ``The correct score should be computed for a board`` () =
        let board =
            { Numbers =
                  [ 14uy
                    21uy
                    17uy
                    24uy
                    4uy
                    10uy
                    16uy
                    15uy
                    9uy
                    19uy
                    18uy
                    8uy
                    23uy
                    26uy
                    20uy
                    22uy
                    11uy
                    13uy
                    6uy
                    5uy
                    2uy
                    0uy
                    12uy
                    3uy
                    7uy ]
              Dim = 5
              Marked =
                  set [ 0
                        1
                        2
                        3
                        4
                        8
                        12
                        16
                        19
                        20
                        21
                        24 ] }

        Assert.Equal(4512u, score 24uy board)
