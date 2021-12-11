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
