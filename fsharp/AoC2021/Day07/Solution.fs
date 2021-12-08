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
    open Xunit

    [<Fact>]
    let ``Test naive fuel cost`` () =
        let points =
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

        Assert.Equal(37u, computeFuelNaive points 2u)

    [<Fact>]
    let ``Test correct fuel cost`` () =
        let points =
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

        Assert.Equal(206u, computeFuelCorrect points 2u)
        Assert.Equal(168u, computeFuelCorrect points 5u)

    let ``Test finding the optimal point`` () =
        let points =
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

        Assert.Equal((2u, 37u), findOptimalPoint computeFuelNaive points)
        Assert.Equal((5u, 168u), findOptimalPoint computeFuelCorrect points)
