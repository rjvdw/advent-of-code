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

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test naive fuel cost`` () =
        ([ 16u
           1u
           2u
           0u
           4u
           2u
           7u
           1u
           2u
           14u ],
         2u)
        ||> computeFuelNaive
        |> should equal 37u

    [<Fact>]
    let ``Test correct fuel cost`` () =
        ([ 16u
           1u
           2u
           0u
           4u
           2u
           7u
           1u
           2u
           14u ],
         2u)
        ||> computeFuelCorrect
        |> should equal 206u

        ([ 16u
           1u
           2u
           0u
           4u
           2u
           7u
           1u
           2u
           14u ],
         5u)
        ||> computeFuelCorrect
        |> should equal 168u

    let ``Test finding the optimal point`` () =
        [ 16u
          1u
          2u
          0u
          4u
          2u
          7u
          1u
          2u
          14u ]
        |> findOptimalPoint computeFuelNaive
        |> should equal (2u, 37u)

        [ 16u
          1u
          2u
          0u
          4u
          2u
          7u
          1u
          2u
          14u ]
        |> findOptimalPoint computeFuelCorrect
        |> should equal (5u, 168u)
