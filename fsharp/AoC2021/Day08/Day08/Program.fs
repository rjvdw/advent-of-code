open System
open System.IO

// https://adventofcode.com/2021/day/8

let args = Environment.GetCommandLineArgs()

if args.Length <> 2 then
    failwith "Usage: $0 <INPUT FILE>"

let input =
    File.ReadLines args.[1] |> Seq.map Solution.parse

printfn $"There are {Solution.countEasyDigits input} easy digits in the output."
printfn $"The sum of all the displays is {Solution.decodeDisplays input}."
