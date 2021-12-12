module Solution

open Instruction

let folder (depth: int, position: int) (instruction: Instruction) =
    match instruction with
    | Forward x -> (depth, position + x)
    | Down x -> (depth + x, position)
    | Up x -> (depth - x, position)

let computeDepthAndPosition (instructions: Instruction seq) = instructions |> Seq.fold folder (0, 0)

let folderWithAim (depth: int, position: int, aim: int) (instruction: Instruction) =
    match instruction with
    | Forward x -> (depth + x * aim, position + x, aim)
    | Down x -> (depth, position, aim + x)
    | Up x -> (depth, position, aim - x)

let computeDepthAndPositionWithAim (instructions: Instruction seq) =
    let depth, position, _ =
        Seq.fold folderWithAim (0, 0, 0) instructions

    (depth, position)
