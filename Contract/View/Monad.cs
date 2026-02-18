namespace Applique.Chronofold.Contract.View;

public record Monad(string Id, double X, double Y, string[] Links, int[] Sequence, int InitialPhase);