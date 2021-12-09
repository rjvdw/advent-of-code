module Solution

let fibSeq =
    (1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL)
    |> Seq.unfold (fun (n0, n1, n2, n3, n4, n5, n6, n7, n8) -> Some(n0, (n1, n2, n3, n4, n5, n6, n7, n8, n0 + n2)))


let solve (days: int) (values: seq<int>) =
    let day n = fibSeq |> Seq.cache |> Seq.item n

    values
    |> Seq.map (fun age -> day (days + 9 - age - 1))
    |> Seq.fold (fun sum count -> sum + count) 0UL

module Tests =
    open FsUnit
    open Xunit

    [<Fact>]
    let ``Test solver`` () =
        let values = [ 3; 4; 3; 1; 2 ]
        values |> solve 0 |> should equal 5UL
        values |> solve 1 |> should equal 5UL
        values |> solve 2 |> should equal 6UL
        values |> solve 3 |> should equal 7UL
        values |> solve 4 |> should equal 9UL
        values |> solve 5 |> should equal 10UL
        values |> solve 80 |> should equal 5934UL
        values |> solve 256 |> should equal 26984457539UL
