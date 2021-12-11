module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Verify that the correct number of increases is counted when the window size is 1.`` () =
    [ 199 ; 200 ; 208 ; 210 ; 200 ; 207 ; 240 ; 269 ; 260 ; 263 ]
    |> countIncreases 1
    |> should equal 7

[<Fact>]
let ``Verify that the correct number of increases is counted when the window size is 3.`` () =
    [ 199 ; 200 ; 208 ; 210 ; 200 ; 207 ; 240 ; 269 ; 260 ; 263 ]
    |> countIncreases 3
    |> should equal 5
