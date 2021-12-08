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
    open Xunit

    [<Fact>]
    let ``Test solver`` () =
        let values = [ 3; 4; 3; 1; 2 ]
        Assert.Equal(5UL, solve 0 values)
        Assert.Equal(5UL, solve 1 values)
        Assert.Equal(6UL, solve 2 values)
        Assert.Equal(7UL, solve 3 values)
        Assert.Equal(9UL, solve 4 values)
        Assert.Equal(10UL, solve 5 values)
        Assert.Equal(5934UL, solve 80 values)
        Assert.Equal(26984457539UL, solve 256 values)
