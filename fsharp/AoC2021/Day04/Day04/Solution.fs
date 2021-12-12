module Solution

type GameResult = { LastNumber: byte; Score: byte }

let rec play (boards: Board.Board list) (numbers: byte list) =
    match numbers with
    | number :: nextNumbers ->
        let nextBoards = boards |> List.map (Board.mark number)

        match nextBoards |> List.tryFind Board.bingo with
        | Some board -> Some(Board.score number board)
        | _ -> play nextBoards nextNumbers
    | _ -> None

let rec findLosingBoard (boards: Board.Board list) (numbers: byte list) =
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
