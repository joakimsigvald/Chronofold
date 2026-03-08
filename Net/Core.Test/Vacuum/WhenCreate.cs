using Applique.Chronofold.Core.Model;
using XspecT;
using XspecT.Assert;
using Xunit;

namespace Applique.Chronofold.Core.Test.Vacuum;

public abstract class WhenCreate : Spec<Core.Model.Vacuum>
{
    private static readonly Tag<int> _depth = new(nameof(_depth));

    protected WhenCreate() => When(_ => Core.Model.Vacuum.Create(The(_depth)));

    public class GivenDepth_0 : WhenCreate
    {
        public GivenDepth_0() => Given(_depth).Is(0);
        [Fact] public void ThenHasOneMonad() => Result.Monads.Has().Count(1);
        [Fact] public void ThenHasNoLinks() => Result.Links.Is().Empty();
    }

    public class GivenDepth_1 : WhenCreate
    {
        public GivenDepth_1() => Given(_depth).Is(1);

        [Fact]
        public void ThenHasSevenMonads_WithRadialIndices()
        {
            Result.Monads.Has().Count(7);
            Result.Monads.Select(m => m.RadialIndex).Is().EqualTo([6, 1, 5, 0, 2, 4, 3]);
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

            var centerMonad = Result.Monads[3];
            Monad[] peripheralMonads = [.. Result.Monads.Except([centerMonad])];
            var links = centerMonad.Links;

            centerMonad.Neighbours.Is().EquivalentTo(peripheralMonads);
            links.Has().Count(6);
            var outwardLinks = links[1..4];
            Link[] inwardLinks = [links[0], ..links[4..]];
            outwardLinks.Select(l => l.Left).Has().All(m => m == centerMonad);
            inwardLinks.Select(l => l.Right).Has().All(m => m == centerMonad);
        }
    }

    public class GivenDifferentDepths : WhenCreate
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

        [Theory]
        [InlineData(1, 1)]
        [InlineData(2, 7)]
        [InlineData(3, 19)]
        public void ThenInnerMonadHaveClockwiseLinks(int depth, int innerMonadCount)
        {
            Given(_depth).Is(depth);
            var monads = Result.Monads;
            Monad[] innerMonads = [.. Result.Monads.OrderBy(m => m.RadialIndex).Take(innerMonadCount)];
            innerMonads.Has().All(IsLinksClockwise);
        }

        private static bool IsLinksClockwise(Monad monad)
        {
            var links = monad.Links;
            return
                links[0].X > monad.X && links[0].Y < monad.Y
                && links[1].X > monad.X && links[1].Y == monad.Y
                && links[2].X > monad.X && links[2].Y > monad.Y
                && links[3].X < monad.X && links[3].Y > monad.Y
                && links[4].X < monad.X && links[4].Y == monad.Y
                && links[5].X < monad.X && links[5].Y < monad.Y;
        }
    }
}