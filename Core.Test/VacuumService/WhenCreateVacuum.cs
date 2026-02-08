using Applique.Chronofold.Contract;
using XspecT;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.VacuumService;

public abstract class WhenCreateVacuum : Spec<Core.VacuumService, Vacuum>
{
    private static readonly Tag<int> _depth = new(nameof(_depth));

    protected WhenCreateVacuum() => When(_ => _.CreateVacuum(The(_depth)));

    public class GivenDepth_0 : WhenCreateVacuum
    {
        public GivenDepth_0() => Given(_depth).Is(0);
        [Fact] public void ThenHasOneMonad() => Result.Monads.Has().Count(1);
        [Fact] public void ThenHasNoLinks() => Result.Links.Is().Empty();
    }

    public class GivenDepth_1 : WhenCreateVacuum
    {
        public GivenDepth_1() => Given(_depth).Is(1);

        [Fact]
        public void ThenHasSevenMonads_WithSuccessiveIds()
        {
            Result.Monads.Has().Count(7);
            Result.Monads.Select(m => m.Id).Is().EqualTo(["7", "2", "6", "1", "3", "5", "4"]);
        }

        [Fact] public void ThenHas_12_Links() => Result.Links.Has().Count(12);

        [Fact]
        public void ThenCenterNodeHas_6_Links_AndOther_3_Links()
        {
            var linkedNodes =
               Result.Links
               .SelectMany(l => new int[] { l.Left.LinearIndex, l.Right.LinearIndex })
               .Order()
               .GroupBy(idx => idx)
               .Select(g => (id: g.Key, count: g.Count()))
               .ToDictionary();
            linkedNodes.Has().Count(7);
            linkedNodes[0].Is(3);
            linkedNodes[1].Is(3);
            linkedNodes[2].Is(3);
            linkedNodes[3].Is(6);
            linkedNodes[4].Is(3);
            linkedNodes[5].Is(3);
            linkedNodes[6].Is(3);
        }
    }

    public class GivenDifferentDepths : WhenCreateVacuum
    {
        [Theory]
        [InlineData(0)]
        [InlineData(1)]
        [InlineData(2)]
        [InlineData(3)]
        [InlineData(4)]
        public void ThenHasMonadsAndLinks(int depth)
        {
            var monads = 3 * depth * (depth + 1) + 1;
            var links = 3 * (monads - 2 * depth - 1);

            Given(_depth).Is(depth);
            Result.Monads.Has().Count(monads);
            Result.Links.Has().Count(links);
        }

        [Theory]
        [InlineData(0, 0, 0)]
        [InlineData(1, 6, 3)]
        [InlineData(2, 18, 12)]
        [InlineData(3, 36, 27)]
        public void ThenMonadHasRadialIndex(int depth, int first, int last)
        {
            Given(_depth).Is(depth);
            var monads = Result.Monads;
            monads[0].RadialIndex.Is(first);
            monads[monads.Length / 2].RadialIndex.Is(0);
            monads[^1].RadialIndex.Is(last);
        }
    }
}