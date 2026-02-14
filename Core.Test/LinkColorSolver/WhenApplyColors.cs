using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
using XspecT;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.LinkColorSolver;

public abstract class WhenApplyColors : Spec<Core.LinkColorSolver>
{
    protected WhenApplyColors() => When(_ => _.ApplyColors());

    public class GivenOneRing : WhenApplyColors
    {
        [Fact]
        public void ThenColorAllLinks()
        {
            const int depth = 1;
            Coordinate[] coordinates = [.. Coordinate.Generate(depth)];
            Monad[] monads = [.. Monad.Generate(coordinates)];
            Link[] links = [.. Link.Generate(depth, coordinates, monads)];
            new NeighbourSolver(monads, links).ApplyNeighbours();
            Given(monads);
            Then();
            monads[3].Links.Select(l => l.Color).Is().EqualTo(LinkColor.Black.Complements);
        }
    }

    public class GivenTwoRings : WhenApplyColors
    {
        [Fact]
        public void ThenColorAllLinks()
        {
            const int depth = 2;
            Coordinate[] coordinates = [.. Coordinate.Generate(depth)];
            Monad[] monads = [.. Monad.Generate(coordinates)];
            Link[] links = [.. Link.Generate(depth, coordinates, monads)];
            new NeighbourSolver(monads, links).ApplyNeighbours();
            Given(monads);
            Then();
            Monad[] radialMonads = [.. monads.OrderBy(m => m.RadialIndex)];
            radialMonads[0].Links.Select(l => l.Color).Is().EqualTo(LinkColor.Black.Complements);
            for (var i = 0; i < 6; i++)
                radialMonads[i + 1].Links[i].Color.Is((LinkColor)(1 << (i + 3) % 6));
        }
    }
}
