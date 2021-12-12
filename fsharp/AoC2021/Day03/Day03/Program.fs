open System
open System.IO

// https://adventofcode.com/2021/day/3

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let lines = File.ReadLines args.[1] |> Seq.toList

let readings =
    List.map (fun line -> Convert.ToUInt16(line, 2)) lines

let len = List.head lines |> String.length

printfn $"The power consumption is {Solution.computePowerConsumption len readings}."
printfn $"The life support rating is {Solution.computeLifeSupportRating len readings}."
