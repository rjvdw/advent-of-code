module SolutionTests

open FsUnit
open Solution
open Xunit

[<Fact>]
let ``Test depth and position without Aim`` () =
    [ "forward 5"
      "down 5"
      "forward 8"
      "up 3"
      "down 8"
      "forward 2" ]
    |> Seq.map Instruction.parse
    |> computeDepthAndPosition
    |> should equal (10, 15)

[<Fact>]
let ``Test depth and position with Aim`` () =
    [ "forward 5"
      "down 5"
      "forward 8"
      "up 3"
      "down 8"
      "forward 2" ]
    |> Seq.map Instruction.parse
    |> computeDepthAndPositionWithAim
    |> should equal (60, 15)
