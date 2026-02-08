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
            Result.Monads.Has().All((_, i) => _.Id == $"{i + 1}");
        }

        [Fact] public void ThenHas_12_Links() => Result.Links.Has().Count(12);

        [Fact]
        public void ThenCenterNodeHas_6_Links_AndOther_3_Links()
        {
            var linkedNodes =
               Result.Links
               .SelectMany(l => new string[] { l.Left.Id, l.Right.Id })
               .Order()
               .GroupBy(id => id)
               .Select(g => (id: g.Key, count: g.Count()))
               .ToDictionary();
            linkedNodes.Has().Count(7);
            linkedNodes["1"].Is(3);
            linkedNodes["2"].Is(3);
            linkedNodes["3"].Is(3);
            linkedNodes["4"].Is(6);
            linkedNodes["5"].Is(3);
            linkedNodes["6"].Is(3);
            linkedNodes["7"].Is(3);
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
    }
}