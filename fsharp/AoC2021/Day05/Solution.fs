module Solution

type Point =
    { X: int; Y: int }
    static member parse (str: string) =
        let idx = str.IndexOf(',')
        if idx = -1 then failwith $"Invalid input: {str}"
        { X = int str[..idx - 1]; Y = int str[(idx + 1)..] }

let asSeq x y =
    if x < y then
        { x .. y }
    elif x > y then
        { x .. -1 .. y }
    else
        Seq.initInfinite (fun _ -> x)

type Line =
    { P1: Point; P2: Point }
    static member parse (str: string) =
        let idx = str.IndexOf(" -> ")
        if idx = -1 then failwith $"Invalid input: {str}"
        { P1 = Point.parse str[..idx - 1]; P2 = Point.parse str[(idx + 4)..] }
    member this.isStraight =
        this.P1.X = this.P2.X || this.P1.Y = this.P2.Y
    member this.points =
        (asSeq this.P1.X this.P2.X, asSeq this.P1.Y this.P2.Y ) ||> Seq.map2 (fun x y -> { X = x; Y = y })

let countDangerousPoints (includeDiagonals: bool) (lines: seq<Line>) =
    lines
    |> Seq.filter (fun line -> includeDiagonals || line.isStraight)
    |> Seq.collect (fun line -> line.points)
    |> Seq.groupBy id
    |> Seq.map (fun (_, points) -> points |> Seq.length)
    |> Seq.filter (fun x -> x > 1)
    |> Seq.length

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test the correct number of dangerous points are counted (excluding diagonals)`` () =
        [ "0,9 -> 5,9"
          "8,0 -> 0,8"
          "9,4 -> 3,4"
          "2,2 -> 2,1"
          "7,0 -> 7,4"
          "6,4 -> 2,0"
          "0,9 -> 2,9"
          "3,4 -> 1,4"
          "0,0 -> 8,8"
          "5,5 -> 8,2" ]
        |> List.map Line.parse
        |> countDangerousPoints false
        |> should equal 5

    [<Fact>]
    let ``Test the correct number of dangerous points are counted (including diagonals)`` () =
        [ "0,9 -> 5,9"
          "8,0 -> 0,8"
          "9,4 -> 3,4"
          "2,2 -> 2,1"
          "7,0 -> 7,4"
          "6,4 -> 2,0"
          "0,9 -> 2,9"
          "3,4 -> 1,4"
          "0,0 -> 8,8"
          "5,5 -> 8,2" ]
        |> List.map Line.parse
        |> countDangerousPoints true
        |> should equal 12
