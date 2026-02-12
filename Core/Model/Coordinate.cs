namespace Applique.Chronofold.Core.Model;

public record Coordinate(int Row, int Col, int Width)
{
    private static readonly double _sqrt3 = Math.Sqrt(3);

    internal int X { get; } = 2 * Col + 1 - Width;
    internal double Y { get; } = _sqrt3 * Row;

    public static IEnumerable<Coordinate> Generate(int depth)
    {
        for (int row = -depth; row <= depth; row++)
        {
            int width = 2 * depth + 1 - Math.Abs(row);
            for (int col = 0; col < width; col++)
                yield return new(row, col, width);
        }
    }

    internal int ComputeRadialIndex()
    {
        var ring = ComputeRing();
        return ring == 0 
            ? 0 
            : CountInside() + ComputeStep();

        int ComputeRing()
        {
            var ra = Math.Abs(Row);
            var ca = Math.Abs(X);
            return (Math.Max(ra, ca) + ra) / 2;
        }

        int ComputeStep()
        {
            if (Row <= 0 && X > Row)
                return (X + Row) / 2 + ring;
            if (X > 0 && X >= Row)
                return X + 2 * Row;
            if (-X <= Row)
                return (Row - X) / 2 + 3 * ring;
            if (Row > 0)
                return Row - 2 * X + ring;
            return -X - 2 * Row + 3 * ring;
        }

        int CountInside() => 3 * ring * (ring - 1);
    }
}