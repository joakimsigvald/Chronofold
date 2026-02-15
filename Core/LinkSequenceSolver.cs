using Applique.Chronofold.Core.Model;

namespace Applique.Chronofold.Core;

public class LinkSequenceSolver(Monad[] monads)
{
    public void Solve()
    {
        int[][] startingSequences = [.. GetAllPermutations()];

        foreach (var monad in monads)
            monad.Sequence = monad.PhaseShift == 0 ? [0, 3, 1, 4, 2, 5] : [0, 3, 1, 4, 5, 2];
    }

    private static IEnumerable<int[]> GetAllPermutations()
    {
        int[] nums = [1, 2, 3, 4, 5];
        return from b in nums[..3]
               from c in b == 3 ? [1, 2] : nums.Except([b])
               from d in nums.Except([b, c])
               from e in nums.Except([b, c, d])
               select new[] { 0, b, c, d, e, nums.Except([b, c, d, e]).Single() };
    }
}