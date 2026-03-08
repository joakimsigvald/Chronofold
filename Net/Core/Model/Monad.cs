using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public class Monad(int linearIndex, int radialIndex, double x = 0, double y = 0)
{
    public static readonly Monad NoMonad = new(-1, -1);

    public int LinearIndex => linearIndex;
    public int RadialIndex => radialIndex;
    public double X => x;
    public double Y => y;

    public Link[] Links { get; set; } = [];
    public Monad[] Neighbours => [.. Links.Select(l => l.OtherHalf(this))];

    public LinkColor Color { get; set; }
    public int[] Sequence { get; set; } = [0, 1, 2, 3, 4, 5];
    public string Id => $"{RadialIndex + 1}";

    public override string ToString() 
        => $"({RadialIndex}: {Color} = {string.Join('-', Links.Select(l => l.Color))})";

    public static IEnumerable<Monad> Generate(Coordinate[] coordinates) => coordinates.Select(Create);

    internal void Blend(LinkColor color) => Color |= color;
    internal void Unblend(LinkColor color) => Color &= ~color;
    internal Link Opposite(Link link) => IsOnTheEdge ? Link.NoLink : Links[OppositeIndex(link)];

    internal int OppositeIndex(Link link) 
        => (Links.IndexOf(link) + LinkColorExtensions.PaletteSize / 2) % LinkColorExtensions.PaletteSize;

    internal bool IsOnTheEdge => Links.Length < LinkColorExtensions.PaletteSize;

    private static Monad Create(Coordinate c, int index) => new(index, c.ComputeRadialIndex(), c.X, c.Y);
}