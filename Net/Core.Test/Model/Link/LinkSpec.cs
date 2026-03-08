using Applique.Chronofold.Contract;
using Applique.Chronofold.Core.Model;
using XspecT;

namespace Applique.Chronofold.Core.Test.Model.Link;

public abstract class LinkSpec<TResult> : Spec<TResult>
{
    protected static readonly Tag<LinkColor> _linkColor = new(nameof(_linkColor));

    protected static readonly Tag<Monad>
        _leftMonad = new(nameof(_leftMonad)),
        _rightMonad = new(nameof(_rightMonad));

    protected Core.Model.Link CreateLink() => new(0, The(_leftMonad), The(_rightMonad));
    protected Monad CreateMonad(LinkColor color) => new(0, 0) { Color = color};
}