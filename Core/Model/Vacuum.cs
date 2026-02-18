namespace Applique.Chronofold.Core.Model;

public class Vacuum(Monad[] monads, Link[] links) 
{
    public Monad[] Monads => monads;
    public Link[] Links => links;

    public static Vacuum Create(int depth)
    {
        Coordinate[] coordinates = [.. Coordinate.Generate(depth)];
        Monad[] monads = [.. Monad.Generate(coordinates)];
        Monad[] radialMonads = [.. monads.OrderBy(m => m.RadialIndex)];
        Link[] links = [.. Link.Generate(depth, coordinates, monads)];
        new NeighbourSolver(monads, links).Solve();
        new LinkColorSolver(radialMonads).Solve();
        //new LinkSequenceSolver(radialMonads).Solve();
        new SimpleSequenceInitializer(monads).Solve();
        return new(monads, links);
    }
}