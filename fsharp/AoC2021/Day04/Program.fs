open System
open System.IO

// https://adventofcode.com/2021/day/4

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith $"Usage: {args[0]} <INPUT FILE>"

let lines = File.ReadLines(args[1]) |> List.ofSeq

let parse (lines: list<string>) =
    let numbersLine, boardLines =
        match lines with
        | numbers :: "" :: boards -> (numbers, boards)
        | _ -> failwith "Invalid input"

    let numbers =
        numbersLine.Split(',')
        |> Seq.map uint8
        |> List.ofSeq

    let boards = boardLines |> Board.parseBoards

    (numbers, boards)

let numbers, boards = parse lines

match Solution.play boards numbers with
| Some (score) -> printfn $"The score of the winning board is {score}."
| _ -> eprintfn "No board will win with these numbers."

match Solution.findLosingBoard boards numbers with
| Some (score) -> printfn $"The score of the losing board is {score}."
| _ -> eprintfn "No board will win with these numbers."