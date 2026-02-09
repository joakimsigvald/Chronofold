namespace Applique.Chronofold.Core.Model;

public record Monad(int LinearIndex, int RadialIndex, double X, double Y)
{
    public Link[] Links { get; internal set; } = [];
    public Monad[] Neighbours { get; internal set; } = [];
}