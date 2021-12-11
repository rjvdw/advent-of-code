module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Test solver`` () =
    let values = [ 3; 4; 3; 1; 2 ]
    values |> solve 0 |> should equal 5UL
    values |> solve 1 |> should equal 5UL
    values |> solve 2 |> should equal 6UL
    values |> solve 3 |> should equal 7UL
    values |> solve 4 |> should equal 9UL
    values |> solve 5 |> should equal 10UL
    values |> solve 80 |> should equal 5934UL
    values |> solve 256 |> should equal 26984457539UL
