module Solution

let maskSeq =
    Seq.unfold (fun x -> Some(x, x * 2us)) 0b1us

let countBits (mask: uint16) =
    Seq.map (fun x -> x &&& mask)
    >> Seq.fold
        (fun (c0, c1) x ->
            if x = 0us then
                (c0 + 1, c1)
            else
                (c0, c1 + 1))
        (0, 0)

let computePowerConsumption len (values: uint16 seq) =
    let g, e =
        maskSeq
        |> Seq.take len
        |> Seq.map (fun mask -> (mask, countBits mask values))
        |> Seq.fold
            (fun (g, e) (mask, (c0, c1)) ->
                if c0 > c1 then (g, e ||| mask)
                elif c0 = c1 then (g, e)
                else (g ||| mask, e))
            (0us, 0us)

    (uint32 g) * (uint32 e)

let rec filterRatings p (mask: uint16) (values: uint16 list) =
    match values with
    | _ :: _ :: _ ->
        let bit =
            if p (countBits mask values) then
                mask
            else
                0us

        let nextMask = mask / 2us

        let nextList =
            List.filter (fun v -> (v &&& mask) = bit) values

        filterRatings p nextMask nextList
    | head :: _ -> uint32 head
    | _ -> failwith "Empty ratings list"

let computeLifeSupportRating len (values: uint16 list) =
    let mask = maskSeq |> Seq.item (len - 1)

    let oxy =
        values
        |> filterRatings (fun (c0, c1) -> c1 >= c0) mask

    let co2 =
        values
        |> filterRatings (fun (c0, c1) -> c1 < c0) mask

    oxy * co2
