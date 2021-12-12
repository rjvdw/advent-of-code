module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Verify that the solution works.`` () =
    [ "Line 1" ; "Line 2" ]
    |> solve
    |> should equal "Hello, World! [\"Line 1\"; \"Line 2\"]"
