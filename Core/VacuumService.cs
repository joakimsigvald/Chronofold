using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    const int _range = 4; // Three layers around the center

    public VacuumState CreateVacuum()
    {
        Monad[] monads = [.. CreateMonads()];
        Link[] links = [.. CreateLinks(monads)];
        return new(monads, links);
    }

    private static IEnumerable<Monad> CreateMonads()
    {
        int count = 0;
        for (int row = -_range; row <= _range; row++)
        {
            int width = 2 * _range + 1 - Math.Abs(row);
            double xOffset = 1 - width;
            for (int col = 0; col < width; col++)
                yield return new($"{++count}", 2 * col + xOffset, Math.Sqrt(3) * row);
        }
    }

    private static IEnumerable<Link> CreateLinks(Monad[] monads)
    {
        int count = 0;
        for (int row = -_range; row <= _range; row++)
        {
            int width = 2 * _range + 1 - Math.Abs(row);
            for (int col = 0; col < width; col++)
            {
                var index = count++;
                var offset = row < 0 ? width : width - 1;
                if (col < width - 1)
                    yield return new(monads[index], monads[index + 1]);
                if (row == _range)
                    continue;
                if (col > 0 || row < 0)
                    yield return new(monads[index], monads[index + offset]);
                if (col < width - 1 || row < 0)
                    yield return new(monads[index], monads[index + offset + 1]);
            }
        }
    }
}