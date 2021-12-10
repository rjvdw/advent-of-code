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

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test [({(<(())[]>[[{[]{<()<>>`` () =
        let result = parse "[({(<(())[]>[[{[]{<()<>>"
        result |> should equal (Valid "}}]])})]")
        result |> score |> should equal 288957UL

    [<Fact>]
    let ``Test [(()[<>])]({[<{<<[]>>(`` () =
        let result = parse "[(()[<>])]({[<{<<[]>>("
        result |> should equal (Valid ")}>]})")
        result |> score |> should equal 5566UL

    [<Fact>]
    let ``Test {([(<{}[<>[]}>{[]{[(<()>`` () =
        let result = parse "{([(<{}[<>[]}>{[]{[(<()>"
        result |> should equal (Invalid '}')
        result |> score |> should equal 1197UL

    [<Fact>]
    let ``Test (((({<>}<{<{<>}{[]{[]{}`` () =
        let result = parse "(((({<>}<{<{<>}{[]{[]{}"
        result |> should equal (Valid "}}>}>))))")
        result |> score |> should equal 1480781UL

    [<Fact>]
    let ``Test [[<[([]))<([[{}[[()]]]`` () =
        let result = parse "[[<[([]))<([[{}[[()]]]"
        result |> should equal (Invalid ')')
        result |> score |> should equal 3UL

    [<Fact>]
    let ``Test [{[{({}]{}}([{[{{{}}([]`` () =
        let result = parse "[{[{({}]{}}([{[{{{}}([]"
        result |> should equal (Invalid ']')
        result |> score |> should equal 57UL

    [<Fact>]
    let ``Test {<[[]]>}<{[{[{[]{()[[[]`` () =
        let result = parse "{<[[]]>}<{[{[{[]{()[[[]"
        result |> should equal (Valid "]]}}]}]}>")
        result |> score |> should equal 995444UL

    [<Fact>]
    let ``Test [<(<(<(<{}))><([]([]()`` () =
        let result = parse "[<(<(<(<{}))><([]([]()"
        result |> should equal (Invalid ')')
        result |> score |> should equal 3UL

    [<Fact>]
    let ``Test <{([([[(<>()){}]>(<<{{`` () =
        let result = parse "<{([([[(<>()){}]>(<<{{"
        result |> should equal (Invalid '>')
        result |> score |> should equal 25137UL

    [<Fact>]
    let ``Test <{([{{}}[<[[[<>{}]]]>[]]`` () =
        let result = parse "<{([{{}}[<[[[<>{}]]]>[]]"
        result |> should equal (Valid "])}>")
        result |> score |> should equal 294UL
