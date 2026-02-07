namespace Applique.Chronofold.Contract;

public record Link(Monad Left, Monad Right) 
{
    public double X => (Left.X + Right.X) / 2;
    public double Y => (Left.Y + Right.Y) / 2;
}