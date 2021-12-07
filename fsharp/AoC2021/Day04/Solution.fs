module Solution

type GameResult = { LastNumber: uint8; Score: uint8 }

let rec play (boards: list<Board.Board>) (numbers: list<uint8>) =
    match numbers with
    | number :: nextNumbers ->
        let nextBoards = boards |> List.map (Board.mark number)

        match nextBoards |> List.tryFind Board.bingo with
        | Some board -> Some(Board.score number board)
        | _ -> play nextBoards nextNumbers
    | _ -> None

let rec findLosingBoard (boards: list<Board.Board>) (numbers: list<uint8>) =
    match numbers with
    | number :: nextNumbers ->
        let markedBoards = boards |> List.map (Board.mark number)

        let nextBoards =
            markedBoards
            |> List.filter (fun board -> not (Board.bingo board))

        match findLosingBoard nextBoards nextNumbers with
        | None ->
            match markedBoards |> List.tryFind Board.bingo with
            | Some board -> Some(Board.score number board)
            | _ -> None
        | x -> x
    | _ -> None

module Tests =
    open Board
    open Xunit

    [<Fact>]
    let ``The correct score for the winning board should be found when playing.`` () =
        let numbers =
            List.map
                uint8
                [ 7
                  4
                  9
                  5
                  11
                  17
                  23
                  2
                  0
                  14
                  21
                  24
                  10
                  16
                  13
                  6
                  15
                  25
                  12
                  22
                  18
                  20
                  8
                  19
                  3
                  26
                  1 ]

        let boards =
            [ { Numbers =
                    List.map
                        uint8
                        [ 22
                          13
                          17
                          11
                          0
                          8
                          2
                          23
                          4
                          24
                          21
                          9
                          14
                          16
                          7
                          6
                          10
                          3
                          18
                          5
                          1
                          12
                          20
                          15
                          19 ]
                Dim = 5
                Marked = Set.empty }
              { Numbers =
                    List.map
                        uint8
                        [ 3
                          15
                          0
                          2
                          22
                          9
                          18
                          13
                          17
                          5
                          19
                          8
                          7
                          25
                          23
                          20
                          11
                          10
                          24
                          4
                          14
                          21
                          16
                          12
                          6 ]
                Dim = 5
                Marked = Set.empty }
              { Numbers =
                    List.map
                        uint8
                        [ 14
                          21
                          17
                          24
                          4
                          10
                          16
                          15
                          9
                          19
                          18
                          8
                          23
                          26
                          20
                          22
                          11
                          13
                          6
                          5
                          2
                          0
                          12
                          3
                          7 ]
                Dim = 5
                Marked = Set.empty } ]

        Assert.Equal(Some(uint32 4512), play boards numbers)

    [<Fact>]
    let ``The correct score for the losing board should be found when playing.`` () =
        let numbers =
            List.map
                uint8
                [ 7
                  4
                  9
                  5
                  11
                  17
                  23
                  2
                  0
                  14
                  21
                  24
                  10
                  16
                  13
                  6
                  15
                  25
                  12
                  22
                  18
                  20
                  8
                  19
                  3
                  26
                  1 ]

        let boards =
            [ { Numbers =
                    List.map
                        uint8
                        [ 22
                          13
                          17
                          11
                          0
                          8
                          2
                          23
                          4
                          24
                          21
                          9
                          14
                          16
                          7
                          6
                          10
                          3
                          18
                          5
                          1
                          12
                          20
                          15
                          19 ]
                Dim = 5
                Marked = Set.empty }
              { Numbers =
                    List.map
                        uint8
                        [ 3
                          15
                          0
                          2
                          22
                          9
                          18
                          13
                          17
                          5
                          19
                          8
                          7
                          25
                          23
                          20
                          11
                          10
                          24
                          4
                          14
                          21
                          16
                          12
                          6 ]
                Dim = 5
                Marked = Set.empty }
              { Numbers =
                    List.map
                        uint8
                        [ 14
                          21
                          17
                          24
                          4
                          10
                          16
                          15
                          9
                          19
                          18
                          8
                          23
                          26
                          20
                          22
                          11
                          13
                          6
                          5
                          2
                          0
                          12
                          3
                          7 ]
                Dim = 5
                Marked = Set.empty } ]

        Assert.Equal(Some(uint32 1924), findLosingBoard boards numbers)
