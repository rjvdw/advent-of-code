open System
open System.IO

// https://adventofcode.com/2021/day/6

let args = Environment.GetCommandLineArgs()

if args.Length <> 3 then
    failwith "Usage: $0 <INPUT FILE> <NR DAYS>"

let input = File.ReadLines args.[1] |> Seq.head
let values = input.Split(',') |> Array.map int
let days = int args.[2]

printfn $"After {days} days, there are {Solution.solve days values} lantern fish."
