using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Vacuum CreateVacuum(int depth)
    {
        Coordinate[] coordinates = [.. GetCoordinates()];
        Monad[] monads = [.. coordinates.Select(CreateMonad)];
        Link[] links = [.. coordinates.SelectMany(CreateLinks)];
        return new(monads, links);

        IEnumerable<Coordinate> GetCoordinates()
        {
            int count = 0;
            for (int row = -depth; row <= depth; row++)
            {
                int width = 2 * depth + 1 - Math.Abs(row);
                for (int col = 0; col < width; col++) 
                    yield return new(count++, row, col, width);
            }
        }

        IEnumerable<Link> CreateLinks(Coordinate c)
        {
            var index = c.LinearIndex;
            if (c.Col < c.Width - 1)
                yield return new(monads[c.LinearIndex], monads[index + 1]);
            if (c.Row == depth)
                yield break;

            var offset = c.Row < 0 ? c.Width : c.Width - 1;
            if (c.Col > 0 || c.Row < 0)
                yield return new(monads[index], monads[index + offset]);
            if (c.Col < c.Width - 1 || c.Row < 0)
                yield return new(monads[index], monads[index + offset + 1]);
        }
    }

    private static Monad CreateMonad(Coordinate c) => new(c.LinearIndex, c.ComputeRadialIndex(), c.X, c.Y);
}