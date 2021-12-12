module Solution

let diff (a: uint) (b: uint) = if a > b then a - b else b - a

let computeFuelNaive (points: uint seq) (point: uint) =
    points
    |> Seq.map (fun p -> diff p point)
    |> Seq.sum

let computeFuelCorrect (points: uint seq) (point: uint) =
    points
    |> Seq.map (fun p -> diff p point)
    |> Seq.map (fun d -> d * (d + 1u) / 2u)
    |> Seq.sum

let findOptimalPoint (computeFuel: uint seq -> uint -> uint) (points: uint seq) =
    let min = points |> Seq.min
    let max = points |> Seq.max
    let cost = computeFuel points

    { min .. max }
    |> Seq.map (fun point -> (point, cost point))
    |> Seq.minBy snd
