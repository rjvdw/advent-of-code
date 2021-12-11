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
