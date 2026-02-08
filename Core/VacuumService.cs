using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public Vacuum CreateVacuum(int depth)
    {
        Monad[] monads = [.. CreateMonads()];
        Link[] links = [.. CreateLinks(monads)];
        return new(monads, links);

        IEnumerable<Monad> CreateMonads()
        {
            int count = 0;
            for (int row = -depth; row <= depth; row++)
            {
                int width = 2 * depth + 1 - Math.Abs(row);
                double xOffset = 1 - width;
                for (int col = 0; col < width; col++)
                    yield return new($"{++count}", 2 * col + xOffset, Math.Sqrt(3) * row);
            }
        }

        IEnumerable<Link> CreateLinks(Monad[] monads)
        {
            int count = 0;
            for (int row = -depth; row <= depth; row++)
            {
                int width = 2 * depth + 1 - Math.Abs(row);
                for (int col = 0; col < width; col++)
                {
                    var index = count++;
                    var offset = row < 0 ? width : width - 1;
                    if (col < width - 1)
                        yield return new(monads[index], monads[index + 1]);
                    if (row == depth)
                        continue;
                    if (col > 0 || row < 0)
                        yield return new(monads[index], monads[index + offset]);
                    if (col < width - 1 || row < 0)
                        yield return new(monads[index], monads[index + offset + 1]);
                }
            }
        }
    }
}