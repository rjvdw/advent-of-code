module Solution

let diff (a: uint) (b: uint) = if a > b then a - b else b - a

let computeFuelNaive (points: seq<uint>) (point: uint) =
    points
    |> Seq.map (fun p -> diff p point)
    |> Seq.sum

let computeFuelCorrect (points: seq<uint>) (point: uint) =
    points
    |> Seq.map (fun p -> diff p point)
    |> Seq.map (fun d -> d * (d + 1u) / 2u)
    |> Seq.sum

let findOptimalPoint (computeFuel: seq<uint> -> uint -> uint) (points: seq<uint>) =
    let min = points |> Seq.min
    let max = points |> Seq.max
    let cost = computeFuel points

    { min .. max }
    |> Seq.map (fun point -> (point, cost point))
    |> Seq.minBy snd
