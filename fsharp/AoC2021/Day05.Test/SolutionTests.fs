module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Test the correct number of dangerous points are counted (excluding diagonals)`` () =
    [ "0,9 -> 5,9"
      "8,0 -> 0,8"
      "9,4 -> 3,4"
      "2,2 -> 2,1"
      "7,0 -> 7,4"
      "6,4 -> 2,0"
      "0,9 -> 2,9"
      "3,4 -> 1,4"
      "0,0 -> 8,8"
      "5,5 -> 8,2" ]
    |> List.map Line.parse
    |> countDangerousPoints false
    |> should equal 5

[<Fact>]
let ``Test the correct number of dangerous points are counted (including diagonals)`` () =
    [ "0,9 -> 5,9"
      "8,0 -> 0,8"
      "9,4 -> 3,4"
      "2,2 -> 2,1"
      "7,0 -> 7,4"
      "6,4 -> 2,0"
      "0,9 -> 2,9"
      "3,4 -> 1,4"
      "0,0 -> 8,8"
      "5,5 -> 8,2" ]
    |> List.map Line.parse
    |> countDangerousPoints true
    |> should equal 12
