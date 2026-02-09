using Applique.Chronofold.Core.Model;
namespace Applique.Chronofold.Core;

public static class VacuumGenerator
{
    public static Vacuum CreateVacuum(int depth)
    {
        Coordinate[] coordinates = [.. GetCoordinates()];
        Monad[] monads = [.. coordinates.Select(CreateMonad)];
        Link[] links = CreateAllLinks();
        var solver = new VacuumSolver(monads, links);
        solver.ApplyNeighbours();
        solver.ApplyColors();
        return new(monads, links);

        IEnumerable<Coordinate> GetCoordinates()
        {
            for (int row = -depth; row <= depth; row++)
            {
                int width = 2 * depth + 1 - Math.Abs(row);
                for (int col = 0; col < width; col++)
                    yield return new(row, col, width);
            }
        }

        Link[] CreateAllLinks()
        {
            int count = 0;
            return [.. coordinates.SelectMany(CreateLinks)];

            IEnumerable<Link> CreateLinks(Coordinate c, int index)
            {
                if (c.Col < c.Width - 1)
                    yield return new(count++, monads[index], monads[index + 1]);
                if (c.Row == depth)
                    yield break;

                var offset = c.Row < 0 ? c.Width : c.Width - 1;
                if (c.Col < c.Width - 1 || c.Row < 0)
                    yield return new(count++, monads[index], monads[index + offset + 1]);
                if (c.Col > 0 || c.Row < 0)
                    yield return new(count++, monads[index], monads[index + offset]);
            }
        }
    }

    private static Monad CreateMonad(Coordinate c, int index) => new(index, c.ComputeRadialIndex(), c.X, c.Y);
}