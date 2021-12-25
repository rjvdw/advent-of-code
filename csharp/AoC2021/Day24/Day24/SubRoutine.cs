namespace Day24;

public record SubRoutine(bool Divides, int ConstantX, int ConstantY)
{
    public int ComputeX(int z) => z % 26 + ConstantX;

    public int Run(int w, int z)
    {
        var x = ComputeX(z);

        if (Divides)
            z /= 26;

        if (x != w)
            z = z * 26 + w + ConstantY;

        return z;
    }
}
