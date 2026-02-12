using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core.Model;

public record Monad(int LinearIndex, int RadialIndex, double X = 0, double Y = 0)
{
    public static readonly Monad NoMonad = new(-1, -1);

    public Link[] Links { get; set; } = [];
    public Monad[] Neighbours { get; set; } = [];
    public LinkColor Color { get; private set; }

    public override string ToString() => $"{RadialIndex}: {string.Join('-', Links.Select(l => l.Color))}";

    public static IEnumerable<Monad> Generate(Coordinate[] coordinates) => coordinates.Select(Create);

    private static Monad Create(Coordinate c, int index) => new(index, c.ComputeRadialIndex(), c.X, c.Y);

    internal void Blend(LinkColor color) => Color |= color;
    internal void Unblend(LinkColor color) => Color &= ~color;
    internal Link Opposite(Link link) 
        => IsOnTheEdge 
        ? Link.NoLink 
        : Links[(Links.IndexOf(link) + LinkColorExtensions.PaletteSize / 2) % LinkColorExtensions.PaletteSize];

    private bool IsOnTheEdge => Links.Length < LinkColorExtensions.PaletteSize;
}