open System
open System.IO

// https://adventofcode.com/2021/day/1

let args = Environment.GetCommandLineArgs()

if args.Length <> 3 then
    failwith "Usage: $0 <INPUT FILE> <WINDOW SIZE>"

let numbers = File.ReadLines args.[1] |> Seq.map int

let windowSize = int args.[2]

printfn $"{Solution.countIncreases windowSize numbers}"
