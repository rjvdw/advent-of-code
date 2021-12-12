using System;
using Xunit;

namespace Day08.Test;

public class DisplayTest
{
    private readonly byte[] _defaultMapping =
    {
        Display.A, Display.B, Display.C, Display.D, Display.E, Display.F, Display.G
    };

    [Fact]
    public void TestGetOutput1()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.A | Display.B | Display.C | Display.E | Display.F | Display.G,
                Display.C | Display.F,
                Display.A | Display.C | Display.D | Display.E | Display.G,
                Display.A | Display.C | Display.D | Display.F | Display.G
            });

        Assert.Equal(123U, display.GetOutput(_defaultMapping));
    }

    [Fact]
    public void TestGetOutput2()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.B | Display.C | Display.D | Display.F,
                Display.A | Display.B | Display.D | Display.F | Display.G,
                Display.A | Display.B | Display.D | Display.E | Display.F | Display.G,
                Display.A | Display.C | Display.F
            });

        Assert.Equal(4567U, display.GetOutput(_defaultMapping));
    }

    [Fact]
    public void TestGetOutput3()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.A | Display.B | Display.C | Display.D | Display.E | Display.F | Display.G,
                Display.A | Display.B | Display.C | Display.D | Display.E | Display.F | Display.G,
                Display.A | Display.B | Display.C | Display.D | Display.F | Display.G,
                Display.A | Display.B | Display.C | Display.D | Display.F | Display.G
            });

        Assert.Equal(8899U, display.GetOutput(_defaultMapping));
    }

    [Fact]
    public void TestGetOutputInvalid()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.F | Display.G,
                Display.F | Display.G,
                Display.F | Display.G,
                Display.F | Display.G
            });

        Assert.Throws<ArgumentException>(() => display.GetOutput(_defaultMapping));
    }

    [Fact]
    public void TestGetOutputWithValidMapping()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.B | Display.D | Display.E | Display.F | Display.A,
                Display.B | Display.D | Display.E | Display.G | Display.A,
                Display.C | Display.D | Display.E | Display.G,
                Display.B | Display.C | Display.E | Display.G | Display.A
            });
        byte[] mapping =
        {
            Display.B, Display.C, Display.D, Display.E, Display.F, Display.G, Display.A
        };

        Assert.Equal(2345U, display.GetOutput(mapping));
    }

    [Fact]
    public void TestGetOutputWithInvalidMapping()
    {
        var display = new Display(
            new byte[10], // not relevant for this test
            new byte[]
            {
                Display.B | Display.D | Display.E | Display.F | Display.A,
                Display.B | Display.D | Display.E | Display.G | Display.A,
                Display.C | Display.D | Display.E | Display.G,
                Display.B | Display.C | Display.E | Display.G | Display.A
            });
        byte[] mapping =
        {
            Display.A, Display.A, Display.A, Display.A, Display.A, Display.A, Display.A
        };

        Assert.Throws<ArgumentException>(() => display.GetOutput(mapping));
    }
}
