namespace Applique.Chronofold.Core;

internal record Coordinate(int LinearIndex, int Row, int Col, int Width)
{
    private static readonly double _sqrt3 = Math.Sqrt(3);

    internal int X { get; } = 2 * Col + 1 - Width;
    internal double Y { get; } = _sqrt3 * Row;

    internal int ComputeRadialIndex()
    {
        var ring = ComputeRing();
        if (ring == 0)
            return 0;

        var step = ComputeStep();
        return CountInside() + step;

        int ComputeRing()
        {
            var ra = Math.Abs(Row);
            var ca = Math.Abs(X);
            return (Math.Max(ra, ca) + ra) / 2;
        }

        int ComputeStep()
        {
            if (Row <= 0 && X > Row)
                return (X + Row) / 2 + ring - 1;
            if (X > 0 && X >= Row)
                return X + 2 * Row - 1;
            if (-X <= Row)
                return (Row - X) / 2 + 3 * ring - 1;
            if (Row > 0)
                return Row - 2 * X + ring - 1;
            return -X - 2 * Row + 3 * ring - 1;
        }

        int CountInside() => 3 * ring * (ring - 1) + 1;
    }
}