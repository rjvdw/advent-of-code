module Solution

type GameResult = { LastNumber: byte; Score: byte }

let rec play (boards: list<Board.Board>) (numbers: list<byte>) =
    match numbers with
    | number :: nextNumbers ->
        let nextBoards = boards |> List.map (Board.mark number)

        match nextBoards |> List.tryFind Board.bingo with
        | Some board -> Some(Board.score number board)
        | _ -> play nextBoards nextNumbers
    | _ -> None

let rec findLosingBoard (boards: list<Board.Board>) (numbers: list<byte>) =
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
            [ 7uy
              4uy
              9uy
              5uy
              11uy
              17uy
              23uy
              2uy
              0uy
              14uy
              21uy
              24uy
              10uy
              16uy
              13uy
              6uy
              15uy
              25uy
              12uy
              22uy
              18uy
              20uy
              8uy
              19uy
              3uy
              26uy
              1uy ]

        let boards =
            [ { Numbers =
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
              { Numbers =
                    [ 3uy
                      15uy
                      0uy
                      2uy
                      22uy
                      9uy
                      18uy
                      13uy
                      17uy
                      5uy
                      19uy
                      8uy
                      7uy
                      25uy
                      23uy
                      20uy
                      11uy
                      10uy
                      24uy
                      4uy
                      14uy
                      21uy
                      16uy
                      12uy
                      6uy ]
                Dim = 5
                Marked = Set.empty }
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
                Marked = Set.empty } ]

        Assert.Equal(Some(4512u), play boards numbers)

    [<Fact>]
    let ``The correct score for the losing board should be found when playing.`` () =
        let numbers =
            [ 7uy
              4uy
              9uy
              5uy
              11uy
              17uy
              23uy
              2uy
              0uy
              14uy
              21uy
              24uy
              10uy
              16uy
              13uy
              6uy
              15uy
              25uy
              12uy
              22uy
              18uy
              20uy
              8uy
              19uy
              3uy
              26uy
              1uy ]

        let boards =
            [ { Numbers =
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
              { Numbers =
                    [ 3uy
                      15uy
                      0uy
                      2uy
                      22uy
                      9uy
                      18uy
                      13uy
                      17uy
                      5uy
                      19uy
                      8uy
                      7uy
                      25uy
                      23uy
                      20uy
                      11uy
                      10uy
                      24uy
                      4uy
                      14uy
                      21uy
                      16uy
                      12uy
                      6uy ]
                Dim = 5
                Marked = Set.empty }
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
                Marked = Set.empty } ]

        Assert.Equal(Some(1924u), findLosingBoard boards numbers)
