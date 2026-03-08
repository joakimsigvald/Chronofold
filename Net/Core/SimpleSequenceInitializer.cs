using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class SimpleSequenceInitializer(Monad[] monads)
{
    public void Solve()
    {
        foreach (var monad in monads)
            monad.Sequence = [0, 1, 2, 3, 4, 5];
    }
}