module SolutionTests

open FsUnit
open Solution
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
