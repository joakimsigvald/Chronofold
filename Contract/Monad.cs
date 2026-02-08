namespace Applique.Chronofold.Contract;

public record Monad(int LinearIndex, int RadialIndex, double X, double Y) 
{
    public string Id => $"{RadialIndex + 1}";
}