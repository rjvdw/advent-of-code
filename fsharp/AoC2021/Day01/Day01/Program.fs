open System
open System.IO

let args = Environment.GetCommandLineArgs()

if args.Length <> 3 then
    failwith "Usage: $0 <INPUT FILE> <WINDOW SIZE>"

let numbers = File.ReadLines args.[1] |> Seq.map int

let windowSize = int args.[2]

printfn $"{Solution.countIncreases windowSize numbers}"
