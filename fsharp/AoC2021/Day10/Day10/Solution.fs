module Solution

type Analysis =
    | Invalid of char
    | Valid of string

let score result =
    match result with
    | Invalid c ->
        match c with
        | ')' -> 3UL
        | ']' -> 57UL
        | '}' -> 1197UL
        | '>' -> 25137UL
        | _ -> failwith $"Invalid character: {c}"
    | Valid stack ->
        stack
        |> Seq.map
            (fun c ->
                match c with
                | ')' -> 1UL
                | ']' -> 2UL
                | '}' -> 3UL
                | '>' -> 4UL
                | _ -> failwith $"Invalid character: {c}")
        |> Seq.fold (fun acc v -> 5UL * acc + v) 0UL

let matching ch =
    match ch with
    | '(' -> ")"
    | '[' -> "]"
    | '{' -> "}"
    | '<' -> ">"
    | _ -> failwith $"Invalid character: {ch}"

let parse (line: string) =
    let rec loop (line: char list) (stack: char list) =
        match line with
        | head :: tail ->
            match head with
            | '('
            | '['
            | '{'
            | '<' -> loop tail (head :: stack)
            | ')'
            | ']'
            | '}'
            | '>' ->
                match stack with
                | '(' :: _ when head <> ')' -> Invalid head
                | '[' :: _ when head <> ']' -> Invalid head
                | '{' :: _ when head <> '}' -> Invalid head
                | '<' :: _ when head <> '>' -> Invalid head
                | _ :: st -> loop tail st
                | _ -> Invalid head
            | _ -> failwith $"Invalid character: {head}"
        | _ -> Valid(stack |> List.map matching |> List.reduce (+))

    (line |> List.ofSeq, List.empty) ||> loop
