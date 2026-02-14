namespace Applique.Chronofold.Core.Model;

public class Vacuum(Monad[] monads, Link[] links) 
{
    public Monad[] Monads => monads;
    public Link[] Links => links;

    public static Vacuum Create(int depth)
    {
        Coordinate[] coordinates = [.. Coordinate.Generate(depth)];
        Monad[] monads = [.. Monad.Generate(coordinates)];
        Link[] links = [.. Link.Generate(depth, coordinates, monads)];
        new NeighbourSolver(monads, links).ApplyNeighbours();
        new LinkColorSolver(monads).ApplyColors();
        return new(monads, links);
    }
}