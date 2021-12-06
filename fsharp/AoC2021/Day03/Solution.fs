module Solution

let maskSeq =
    Seq.unfold (fun x -> Some(x, x * uint16 2)) (uint16 0b1)

let countBits (mask: uint16) =
    Seq.map (fun x -> x &&& mask)
    >> Seq.fold
        (fun (c0, c1) x ->
            if x = uint16 0 then
                (c0 + 1, c1)
            else
                (c0, c1 + 1))
        (0, 0)

let computePowerConsumption len (values: seq<uint16>) =
    let g, e =
        maskSeq
        |> Seq.take len
        |> Seq.map (fun mask -> (mask, countBits mask values))
        |> Seq.fold
            (fun (g, e) (mask, (c0, c1)) ->
                if c0 > c1 then (g, e ||| mask)
                elif c0 = c1 then (g, e)
                else (g ||| mask, e))
            (uint16 0, uint16 0)

    (uint32 g) * (uint32 e)

let rec filterRatings p (mask: uint16) (values: list<uint16>) =
    match values with
    | _ :: _ :: _ ->
        let bit =
            if p (countBits mask values) then
                mask
            else
                uint16 0

        let nextMask = mask / uint16 2

        let nextList =
            List.filter (fun v -> (v &&& mask) = bit) values

        filterRatings p nextMask nextList
    | head :: _ -> uint32 head
    | _ -> failwith "Empty ratings list"

let computeLifeSupportRating len (values: list<uint16>) =
    let mask = maskSeq |> Seq.item (len - 1)

    let oxy =
        values
        |> filterRatings (fun (c0, c1) -> c1 >= c0) mask

    let co2 =
        values
        |> filterRatings (fun (c0, c1) -> c1 < c0) mask

    oxy * co2

module Tests =
    open Xunit

    let testData =
        [ uint16 0b00100
          uint16 0b11110
          uint16 0b10110
          uint16 0b10111
          uint16 0b10101
          uint16 0b01111
          uint16 0b00111
          uint16 0b11100
          uint16 0b10000
          uint16 0b11001
          uint16 0b00010
          uint16 0b01010 ]

    [<Fact>]
    let ``Test mask sequence`` () =
        Assert.Equal(
            [ uint16 1
              uint16 2
              uint16 4
              uint16 8 ],
            maskSeq |> Seq.take 4
        )

    [<Fact>]
    let ``Test bit counter`` () =
        Assert.Equal((5, 7), countBits (uint16 0b10000) testData)
        Assert.Equal((7, 5), countBits (uint16 0b01000) testData)
        Assert.Equal((4, 8), countBits (uint16 0b00100) testData)
        Assert.Equal((5, 7), countBits (uint16 0b00010) testData)
        Assert.Equal((7, 5), countBits (uint16 0b00001) testData)

    [<Fact>]
    let ``Test power consumption`` () =
        Assert.Equal(uint32 198, computePowerConsumption 5 testData)

    [<Fact>]
    let ``Test life support rating`` () =
        Assert.Equal(uint32 230, computeLifeSupportRating 5 testData)
