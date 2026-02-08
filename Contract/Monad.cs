namespace Applique.Chronofold.Contract;

public record Monad(int Index, double X, double Y) 
{
    public string Id => $"{Index + 1}";
}