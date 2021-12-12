module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Test naive fuel cost`` () =
    ([ 16u; 1u; 2u; 0u; 4u; 2u; 7u; 1u; 2u; 14u ], 2u)
    ||> computeFuelNaive
    |> should equal 37u

[<Fact>]
let ``Test correct fuel cost`` () =
    ([ 16u; 1u; 2u; 0u; 4u; 2u; 7u; 1u; 2u; 14u ], 2u)
    ||> computeFuelCorrect
    |> should equal 206u

    ([ 16u; 1u; 2u; 0u; 4u; 2u; 7u; 1u; 2u; 14u ], 5u)
    ||> computeFuelCorrect
    |> should equal 168u

let ``Test finding the optimal point`` () =
    [ 16u; 1u; 2u; 0u; 4u; 2u; 7u; 1u; 2u; 14u ]
    |> findOptimalPoint computeFuelNaive
    |> should equal (2u, 37u)

    [ 16u; 1u; 2u; 0u; 4u; 2u; 7u; 1u; 2u; 14u ]
    |> findOptimalPoint computeFuelCorrect
    |> should equal (5u, 168u)
