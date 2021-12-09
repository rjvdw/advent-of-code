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
            (0us, 0us)

    (uint32 g) * (uint32 e)

let rec filterRatings p (mask: uint16) (values: list<uint16>) =
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
    open FsUnit
    open Xunit

    let testData =
        [ 0b00100us
          0b11110us
          0b10110us
          0b10111us
          0b10101us
          0b01111us
          0b00111us
          0b11100us
          0b10000us
          0b11001us
          0b00010us
          0b01010us ]

    [<Fact>]
    let ``Test mask sequence`` () =
        maskSeq |> Seq.take 4 |> should equal [ 1us ; 2us ; 4us ; 8us ]

    [<Fact>]
    let ``Test bit counter`` () =
        testData |> countBits 0b10000us |> should equal (5, 7)
        testData |> countBits 0b01000us |> should equal (7, 5)
        testData |> countBits 0b00100us |> should equal (4, 8)
        testData |> countBits 0b00010us |> should equal (5, 7)
        testData |> countBits 0b00001us |> should equal (7, 5)

    [<Fact>]
    let ``Test power consumption`` () =
        testData |> computePowerConsumption 5 |> should equal 198u

    [<Fact>]
    let ``Test life support rating`` () =
        testData |> computeLifeSupportRating 5 |> should equal 230u
