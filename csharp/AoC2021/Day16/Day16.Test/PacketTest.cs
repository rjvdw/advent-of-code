using Xunit;

namespace Day16.Test;

public class PacketTest
{
    [Fact]
    public void TestSumVersions()
    {
        Assert.Equal(6, Packet.Parse(new[] { "D2FE28" }).SumVersions());
        Assert.Equal(9, Packet.Parse(new[] { "38006F45291200" }).SumVersions());
        Assert.Equal(14, Packet.Parse(new[] { "EE00D40C823060" }).SumVersions());
        Assert.Equal(16, Packet.Parse(new[] { "8A004A801A8002F478" }).SumVersions());
        Assert.Equal(12, Packet.Parse(new[] { "620080001611562C8802118E34" }).SumVersions());
        Assert.Equal(23, Packet.Parse(new[] { "C0015000016115A2E0802F182340" }).SumVersions());
        Assert.Equal(31, Packet.Parse(new[] { "A0016C880162017C3686B18A3D4780" }).SumVersions());
    }

    [Fact]
    public void TestEval()
    {
        Assert.Equal(2021, Packet.Parse(new[] { "D2FE28" }).Eval());
        Assert.Equal(1, Packet.Parse(new[] { "38006F45291200" }).Eval());
        Assert.Equal(3, Packet.Parse(new[] { "EE00D40C823060" }).Eval());
        Assert.Equal(3, Packet.Parse(new[] { "C200B40A82" }).Eval());
        Assert.Equal(54, Packet.Parse(new[] { "04005AC33890" }).Eval());
        Assert.Equal(7, Packet.Parse(new[] { "880086C3E88112" }).Eval());
        Assert.Equal(9, Packet.Parse(new[] { "CE00C43D881120" }).Eval());
        Assert.Equal(1, Packet.Parse(new[] { "D8005AC2A8F0" }).Eval());
        Assert.Equal(0, Packet.Parse(new[] { "F600BC2D8F" }).Eval());
        Assert.Equal(0, Packet.Parse(new[] { "9C005AC2F8F0" }).Eval());
        Assert.Equal(1, Packet.Parse(new[] { "9C0141080250320F1802104A08" }).Eval());
    }
}
