using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public static class VacuumGenerator
{
    public static Vacuum CreateVacuum(int depth)
    {
        Coordinate[] coordinates = [.. Coordinate.Generate(depth)];
        Monad[] monads = [..Monad.Generate(coordinates)];
        Link[] links = [..Link.Generate(depth, coordinates, monads)];
        new NeighbourSolver(monads, links).ApplyNeighbours();
        new LinkColorSolver(monads).ApplyColors();
        return new(monads, links);
    }
}