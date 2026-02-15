using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class LinkSequenceSolver
{
    private readonly Monad[] _radialMonads;
    private readonly Monad _centerMonad;
    private readonly Monad[] _innerMonads;

    public LinkSequenceSolver(Monad[] monads)
    {
        _radialMonads = [.. monads.OrderBy(m => m.RadialIndex)];
        _centerMonad = _radialMonads[0];
        _innerMonads = [.. _radialMonads[1..].TakeWhile(m => !m.IsOnTheEdge)];
    }

    public void Solve()
    {
        var sequence = GetBestSequence();
        if (sequence.Length == 0)
            throw new ApplicationException("No valid sequence found for this topology!");

        Apply(sequence);
    }

    private int[] GetBestSequence()
    {
        int[][] startingSequences = [.. GetAllPermutations()];
        (string dna, int count)[] runs = [.. startingSequences.Select(s => (dna: GetDna(s), count: Run(s))).Where(t => t.count > 0)];
        return GetSequence(runs.OrderBy(t => t.count).FirstOrDefault().dna);
    }

    private int Run(int[] startingSequence)
    {
        HashSet<string> candidateDnas = [GetDna(startingSequence)];
        _centerMonad.Sequence = startingSequence;
        LinkColor[] coloring = [.. _centerMonad.Sequence.Select(idx => _centerMonad.Links[idx].Color)];
        for (int i = 1; i < _innerMonads.Length; i++)
        {
            var monad = _radialMonads[i];
            var sequence = ApplySequence(monad, coloring);
            if (!IsMonadSequenced(monad) || !IsSequenceCongruent(monad))
                return 0;

            candidateDnas.Add(GetDna(sequence));
        }
        return candidateDnas.Count;
    }

    private void Apply(int[] startingSequence)
    {
        var centerMonad = _radialMonads[0];
        centerMonad.Sequence = startingSequence;
        LinkColor[] coloring = [.. centerMonad.Sequence.Select(idx => centerMonad.Links[idx].Color)];
        foreach (var monad in _radialMonads[1..])
            ApplySequence(monad, coloring);
    }

    private static string GetDna(int[] sequence) => string.Join('-', sequence);
    private static int[] GetSequence(string dna)
        => string.IsNullOrEmpty(dna) ? [] : [.. dna.Split('-').Select(int.Parse)];

    private static int[] ApplySequence(Monad monad, LinkColor[] coloring)
        => monad.Sequence = FindSequence(monad, coloring);

    private static int[] FindSequence(Monad monad, LinkColor[] coloring)
        => [.. coloring.Select((c, i) => Array.FindIndex(monad.Links, l => l.Color == coloring[i]))];

    private static bool IsMonadSequenced(Monad monad)
        => monad.Sequence.All(p => p >= 0);

    private static bool IsSequenceCongruent(Monad monad)
    {
        if (monad.Sequence.Any(p => p < 0))
            return false;

        (int, Monad)[] sequencedNeighbors = [
            .. monad.Neighbours
            .Select((n, i) => (i, n))
            .Where(t => t.n.RadialIndex < monad.RadialIndex)];

        foreach (var (port, neighbor) in sequencedNeighbors)
        {
            int myStep = Array.IndexOf(monad.Sequence, port);
            int portBackToMe = (port + 3) % 6;
            int neighborStep = Array.IndexOf(neighbor.Sequence, portBackToMe);
            if (myStep != neighborStep)
                return false;
        }
        return true;
    }

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