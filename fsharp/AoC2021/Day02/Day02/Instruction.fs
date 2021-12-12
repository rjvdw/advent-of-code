module Instruction

type Instruction =
    | Forward of int
    | Down of int
    | Up of int

let parse (line : string) =
    match line.IndexOf ' ' with
    | -1 -> failwith $"Invalid input line: {line}"
    | p -> match line[..p - 1] with
           | "forward" -> Forward (int line[p..])
           | "down" -> Down (int line[p..])
           | "up" -> Up (int line[p..])
           | instruction -> failwith $"Invalid input line: '{line}' (instruction: '{instruction}')"
