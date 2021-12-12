module Solution

let fibSeq =
    (1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL, 1UL)
    |> Seq.unfold (fun (n0, n1, n2, n3, n4, n5, n6, n7, n8) -> Some(n0, (n1, n2, n3, n4, n5, n6, n7, n8, n0 + n2)))


let solve (days: int) (values: seq<int>) =
    let day n = fibSeq |> Seq.cache |> Seq.item n

    values
    |> Seq.map (fun age -> day (days + 9 - age - 1))
    |> Seq.fold (fun sum count -> sum + count) 0UL
