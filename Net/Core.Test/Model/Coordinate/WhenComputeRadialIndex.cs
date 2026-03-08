using XspecT;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.Model.Coordinate;

public class WhenComputeRadialIndex : Spec<int>
{
    private static readonly Tag<int>
        _row = new(nameof(_row)),
        _col = new(nameof(_col)),
        _width = new(nameof(_width));

    public WhenComputeRadialIndex()
        => When(_ => new Core.Model.Coordinate(The(_row), The(_col), The(_width)).ComputeRadialIndex());

    [Theory]
    [InlineData(0, 0, 0, 1)] //ring: 0
    [InlineData(1, -1, 1, 2)] //ring: 1
    [InlineData(2, 0, 2, 3)] //ring: 1
    [InlineData(3, 1, 1, 2)] //ring: 1
    [InlineData(4, 1, 0, 2)] //ring: 1
    [InlineData(5, 0, 0, 3)] //ring: 1
    [InlineData(6, -1, 0, 2)] //ring: 1
    [InlineData(7, -2, 1, 3)] //ring: 2
    [InlineData(8, -2, 2, 3)] //ring: 2
    [InlineData(9, -1, 3, 4)] //ring: 2
    [InlineData(10, 0, 4, 5)] //ring: 2
    [InlineData(11, 1, 3, 4)] //ring: 2
    [InlineData(12, 2, 2, 3)] //ring: 2
    [InlineData(13, 2, 1, 3)] //ring: 2
    [InlineData(14, 2, 0, 3)] //ring: 2
    [InlineData(15, 1, 0, 4)] //ring: 2
    [InlineData(16, 0, 0, 5)] //ring: 2
    [InlineData(17, -1, 0, 4)] //ring: 2
    [InlineData(18, -2, 0, 3)] //ring: 2
    public void Test(int expected, int row, int col, int width)
        => Given(_row).Is(row)
        .And(_col).Is(col)
        .And(_width).Is(width)
        .Then().Result.Is(expected);
}