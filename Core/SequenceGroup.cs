namespace Applique.Chronofold.Core;

public class SequenceGroup(int[] mainSequence, List<int[]> sequences)
{
    public int[] MainSequence => mainSequence;
    public int Score => ComputeScore();

    private int ComputeScore() => sequences.Select(GetDna).Distinct().Count();

    private static string GetDna(int[] sequence) => string.Join('-', SortPairs([.. sequence]));

    public static int[] SortPairs(int[] copy)
    {
        for (int i = 0; i < copy.Length; i += 2)
            if (copy[i] > copy[i + 1])
                (copy[i + 1], copy[i]) = (copy[i], copy[i + 1]);
        return copy;
    }
}