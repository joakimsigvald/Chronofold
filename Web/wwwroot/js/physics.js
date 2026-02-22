export const CreatePhysics = (vacuum) => {
    const { monads, links, paletteSize } = vacuum;
    const linkMap = Object.fromEntries(links.map(l => [l.id, l]));
    const monadMap = Object.fromEntries(monads.map(m => [m.id, m]));

    const _getLink = monad => {
        const activePort = monad.sequence[monad.phase];
        const linkId = monad.links[activePort];
        return linkMap[linkId];
    };

    const _clear = link => {
        link.isLeftActive = false;
        link.isRightActive = false;
    };

    const _step = monad => {
        _advancePhase(monad);
        _resetMatch(monad);
        monad.isStressed = false;
    };

    const _isBounced = link => link.isLeftActive && !link.isRightActive;

    const _flip = link => {
        const left = link.left;
        link.left = link.right;
        link.right = left;
        const leftMonad = monadMap[link.left];
        const rightMonad = monadMap[link.right];
        leftMonad.charge++;
        rightMonad.charge--;
    };

    const _swapPorts = (monad, slotA, slotB) => {
        monad.isStressed = true;
        const portA = monad.sequence[slotA];
        monad.sequence[slotA] = monad.sequence[slotB];
        monad.sequence[slotB] = portA;
    };

    const _advancePhase = monad => monad.phase = (monad.phase + 1) % paletteSize;
    const _retreatPhase = monad => monad.phase = (monad.phase + paletteSize - 1) % paletteSize;

    const _isSender = (monad, link) => link?.left === monad.id;
    const _hasIncomingSignal = link => link?.isLeftActive;
    const _setMatch = monad => monad.matches[monad.phase] = true;
    const _resetMatch = monad => monad.matches[monad.phase] = false;
    const _wasMatched = monad => monad.matches[monad.phase];

    const _acceptSignal = (monad, link) => {
        link.isRightActive = true;
        _setMatch(monad);
        _setMatch(monadMap[link.left]);
    }

    const _receiveFuture = monad => {
        for (let i = 1; i < paletteSize; i++) {
            _advancePhase(monad);
            const futureLink = _getLink(monad);
            if (futureLink && _hasIncomingSignal(futureLink)) {
                _acceptSignal(monad, futureLink);
                return true;
            }
        }
        return false;
    };

    const _findLatestMismatch = monad => {
        for (let i = 1; i < paletteSize; i++) {
            if (!_wasMatched(monad))
                return;

            _retreatPhase(monad);
        }
    };

    const _adaptSequence = monad => {
        const currentPhase = monad.phase;
        _receiveFuture(monad) || _findLatestMismatch(monad);
        _swapPorts(monad, currentPhase, monad.phase);
        monad.phase = currentPhase; // Always rewind the clock
    };

    const _send = monad => {
        const currentLink = _getLink(monad);
        if (currentLink?.left !== monad.id)
            return; //monad is not the sender of current link

        currentLink.isLeftActive = true;
    };

    const _receive = monad => {
        const currentLink = _getLink(monad);
        if (_isSender(monad, currentLink))
            return;

        if (_hasIncomingSignal(currentLink))
            _acceptSignal(monad, currentLink);
        else
            _adaptSequence(monad);
    };

    const _bounce = () => links.filter(_isBounced).forEach(_flip);

    return {
        advance: () => {
            links.forEach(_clear);
            monads.forEach(_step);
            monads.forEach(_send);
            monads.forEach(_receive);
            _bounce();
        }
    };
};