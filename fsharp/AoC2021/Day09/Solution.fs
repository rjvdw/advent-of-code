module Solution

open System.Collections.Generic

let neighbours (rows: int, cols: int) (idx: int) =
    let row = idx / cols
    let col = idx % cols

    [| (row - 1, col)
       (row, col - 1)
       (row + 1, col)
       (row, col + 1) |]
    |> Array.filter (fun (r, c) -> r >= 0 && r < rows && c >= 0 && c < cols)
    |> Array.map (fun (r, c) -> r * cols + c)

let findLowPoints (dims: int * int) (map: array<byte>) =
    let nb =
        neighbours dims
        >> Array.map (fun i -> map |> Array.item i)

    let isLowPoint (idx: int, value: byte) =
        idx |> nb |> Array.forall (fun v -> v > value)

    map
    |> Seq.indexed
    |> Seq.filter isLowPoint
    |> Seq.map fst

let findBasin (dims: int * int) (map: array<byte>) (point: int) =
    let mutable seen = HashSet<int>()
    seen.Add(point) |> ignore

    let nb =
        neighbours dims
        >> Array.filter
            (fun i ->
                (map |> Array.item i) <> 9uy
                && not (seen.Contains i))

    let rec find (stack: list<int>) =
        match stack with
        | head :: tail ->
            let n = head |> nb |> Array.toList
            n |> List.iter (seen.Add >> ignore)
            1 + find (tail |> List.append n)
        | _ -> 0

    [ point ] |> find

let findBasins (dims: int * int) (map: array<byte>) (points: seq<int>) =
    let find = findBasin dims map
    points |> Seq.map find

let parse (lines: seq<string>) =
    let dims, map =
        lines
        |> Seq.map (Seq.map (fun ch -> (byte ch) - (byte '0')))
        |> Seq.fold (fun ((rows, _), acc) row -> ((rows + 1, row |> Seq.length), row |> Seq.append acc)) ((0, 0), [])

    (dims, map |> Array.ofSeq)
