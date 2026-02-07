using Applique.Chronofold.Contract;

namespace Applique.Chronofold.Core;

public class VacuumService : IVacuumService
{
    public IEnumerable<Monad> CreateMonads()
    {
        int range = 4; // Three layers around the center
        int count = 0;
        for (int row = -range; row <= range; row++)
        {
            int width = 2 * range + 1 - Math.Abs(row);
            double xOffset = 1-width;
            for (int col = 0; col < width; col++)
                yield return new($"{++count}", 2 * col + xOffset, Math.Sqrt(3) * row);
        }
    }
}