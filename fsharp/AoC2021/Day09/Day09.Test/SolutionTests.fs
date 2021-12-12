module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Test parse`` () =
    [ "219"; "398" ]
    |> parse
    |> should equal ((2, 3), [| 2; 1; 9; 3; 9; 8 |])

[<Fact>]
let ``Test findLowPoints`` () =
    [ "2199943210"
      "3987894921"
      "9856789892"
      "8767896789"
      "9899965678" ]
    |> parse
    ||> findLowPoints
    |> should equal [| 1; 9; 22; 46 |]

[<Fact>]
let ``Test findBasins`` () =
    let data =
        [ "2199943210"
          "3987894921"
          "9856789892"
          "8767896789"
          "9899965678" ]
        |> parse

    let lowPoints = data ||> findLowPoints

    lowPoints
    |> (data ||> findBasins)
    |> should equal [| 3; 9; 14; 9 |]
