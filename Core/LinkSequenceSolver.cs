using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class LinkSequenceSolver(Monad[] radialMonads)
{
    private readonly Monad _centerMonad = radialMonads[0];
    private readonly Monad[] _innerMonads = [.. radialMonads[1..].TakeWhile(m => !m.IsOnTheEdge)];

    public void Solve()
    {
        var sequence = GetBestSequence();
        if (sequence.Length == 0)
            throw new ApplicationException("No valid sequence found for this topology!");

        Apply(sequence);
    }

    private int[] GetBestSequence()
    {
        int[][] sequences = [.. GetAllPermutations()];
        SequenceGroup[] runs = [.. sequences
            .Select(Run).Where(g => g.Score > 0)];
        return runs.OrderBy(g => g.Score).FirstOrDefault()?.MainSequence ?? [];
    }

    private SequenceGroup Run(int[] mainSequence)
    {
        List<int[]> candidates = [mainSequence];
        _centerMonad.Sequence = mainSequence;
        LinkColor[] coloring = [.. _centerMonad.Sequence.Select(idx => _centerMonad.Links[idx].Color)];
        for (int i = 1; i < _innerMonads.Length; i++)
        {
            var monad = radialMonads[i];
            var sequence = ApplySequence(monad, coloring);
            if (!IsMonadSequenced(monad) || !IsSequenceCongruent(monad))
                return new(mainSequence, []);

            candidates.Add(sequence);
        }
        return new(mainSequence, candidates);
    }

    private void Apply(int[] startingSequence)
    {
        _centerMonad.Sequence = startingSequence;
        LinkColor[] coloring = [.. _centerMonad.Sequence.Select(idx => _centerMonad.Links[idx].Color)];
        foreach (var monad in radialMonads[1..])
            ApplySequence(monad, coloring);
    }

    private static int[] ApplySequence(Monad monad, LinkColor[] coloring)
        => monad.Sequence = FindSequence(monad, coloring);

    private static int[] FindSequence(Monad monad, LinkColor[] coloring)
        => [.. coloring.Select((c, i) => Array.FindIndex(monad.Links, l => l.Color == coloring[i]))];

    private static bool IsMonadSequenced(Monad monad)
        => monad.Sequence.All(p => p >= 0);

    private static bool IsSequenceCongruent(Monad monad)
        => GetSequencedNeighbors(monad).All(t => AreAligned(t.port, monad, t.neighbor));

    private static (int port, Monad neighbor)[] GetSequencedNeighbors(Monad monad)
        => [
            .. monad.Neighbours
            .Select((n, i) => (i, n))
            .Where(t => t.n.RadialIndex < monad.RadialIndex)];

    private static bool AreAligned(int port, Monad monad, Monad neighbor)
        => Array.IndexOf(monad.Sequence, port) == Array.IndexOf(neighbor.Sequence, (port + 3) % 6);

    private static IEnumerable<int[]> GetAllPermutations()
    {
        int[] nums = [1, 2, 3, 4, 5];
        return from b in nums[..3] //mirror symmetry along y-axis
               from c in b == 3 ? [1, 2] : nums.Except([b]) //mirror symmetry along y-axis
               from d in nums.Except([b, c])
               from e in nums.Except([b, c, d])
               select new[] { 0, b, c, d, e, nums.Except([b, c, d, e]).Single() };
    }
}