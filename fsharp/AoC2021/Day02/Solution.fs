module Solution

open Instruction

let folder (depth: int, position: int) (instruction: Instruction) =
    match instruction with
    | Forward x -> (depth, position + x)
    | Down x -> (depth + x, position)
    | Up x -> (depth - x, position)

let computeDepthAndPosition (instructions: seq<Instruction>) = Seq.fold folder (0, 0) instructions

let folderWithAim (depth: int, position: int, aim: int) (instruction: Instruction) =
    match instruction with
    | Forward x -> (depth + x * aim, position + x, aim)
    | Down x -> (depth, position, aim + x)
    | Up x -> (depth, position, aim - x)

let computeDepthAndPositionWithAim (instructions: seq<Instruction>) =
    let depth, position, _ =
        Seq.fold folderWithAim (0, 0, 0) instructions

    (depth, position)

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test depth and position without Aim`` () =
        [ "forward 5"
          "down 5"
          "forward 8"
          "up 3"
          "down 8"
          "forward 2" ]
        |> Seq.map parse
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
        |> Seq.map parse
        |> computeDepthAndPositionWithAim
        |> should equal (60, 15)
