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
        monad.hasSent = false;
        monad.hasReceived = false;
    };

    const _isBounced = link => link.isLeftActive !== link.isRightActive;

    const _flip = link => {
        if (!link)
            return;

        const left = link.left;
        link.left = link.right;
        link.right = left;
        const leftMonad = monadMap[link.left];
        const rightMonad = monadMap[link.right];
        leftMonad.charge++;
        rightMonad.charge--;
    };

    const _swapPorts = (monad, slotA, slotB) => {
        if (slotA === slotB)
            return;

        const portA = monad.sequence[slotA];
        monad.sequence[slotA] = monad.sequence[slotB];
        monad.sequence[slotB] = portA;
    };

    const _advancePhase = monad => monad.phase = (monad.phase + 1) % paletteSize;
    const _retreatPhase = monad => monad.phase = (monad.phase + paletteSize - 1) % paletteSize;

    const _isSender = (monad, link) => link?.left === monad.id;
    const _hasIncomingSignal = link => link?.isLeftActive;

    const _acceptSignal = (monad, link) => {
        link.isRightActive = true;
        monad.hasReceived = true;
    }

    const _findReceiver = monad => {
        const currentPhase = monad.phase;
        for (let i = 0; i < paletteSize; i++) {
            const currentLink = _getLink(monad);
            if (currentLink && _hasIncomingSignal(currentLink)) {
                _swapPorts(monad, currentPhase, monad.phase);
                monad.phase = currentPhase;
                return currentLink;
            }
            _advancePhase(monad);
        }
        monad.phase = currentPhase;
        return null;
    };

    const _send = monad => {
        const currentLink = _getLink(monad);
        if (currentLink?.left !== monad.id)
            return;

        currentLink.isLeftActive = true;
        monad.hasSent = true;
    };

    const _receive = monad => {
        const receiver = _findReceiver(monad);
        if (receiver)
            _acceptSignal(monad, receiver);
        else
            _flip(_getLink(monad));
    };

    return {
        advance: () => {
            links.forEach(_clear);
            monads.forEach(_step);
            monads.forEach(_send);
            monads.filter(m => !m.hasSent).forEach(_receive);
            links.filter(_isBounced).forEach(_flip);
        }
    };
};