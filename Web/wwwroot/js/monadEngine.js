export const CreateMonadEngine = (vacuum) => {
    const { monads, links, paletteSize } = vacuum;
    const linkMap = Object.fromEntries(links.map(l => [l.id, l]));

    const _getLink = monad => {
        const activePort = monad.sequence[monad.phase];
        const linkId = monad.links[activePort];
        return linkMap[linkId];
    };

    const _clearLink = link => {
        link.isLeftActive = false;
        link.isRightActive = false;
    };

    const _isBounced = link => link.isLeftActive && !link.isRightActive;

    const _flip = link => {
        const left = link.left;
        link.left = link.right;
        link.right = left;
        //console.log('link flip', link.right, link.left);
    };

    const _swapPorts = (monad, slotA, slotB) => {
        const portA = monad.sequence[slotA];
        monad.sequence[slotA] = monad.sequence[slotB];
        monad.sequence[slotB] = portA;
        console.log('monad adapt sequence', monad.sequence);
    };

    const _advancePhase = monad => monad.phase = (monad.phase + 1) % paletteSize;

    const _isSender = (monad, link) => link?.left === monad.id;
    const _hasIncomingSignal = link => link?.isLeftActive;
    const _acceptSignal = link => {
        link.isRightActive = true;
        //console.log('monad receive', link.right, link.id);
    }

    const _adaptSequence = monad => {
        const currentPhase = monad.phase;
        //let targetPhase = (currentPhase + 1) % paletteSize;
        for (let i = 1; i < paletteSize; i++) {
            _advancePhase(monad);
            const futureLink = _getLink(monad);
            if (futureLink && _hasIncomingSignal(futureLink)) {
                _acceptSignal(futureLink);
                //targetPhase = monad.phase;
                break;
            }
        }
        _swapPorts(monad, currentPhase, monad.phase);
        monad.phase = currentPhase; // Always rewind the clock
    };

    const _send = monad => {
        const currentLink = _getLink(monad);
        if (currentLink?.left !== monad.id)
            return;

        //console.log('monad send', monad.id, currentLink.id);
        currentLink.isLeftActive = true;
    };

    const _receive = monad => {
        const currentLink = _getLink(monad);
        if (_isSender(monad, currentLink))
            return;

        if (_hasIncomingSignal(currentLink))
            _acceptSignal(currentLink);
        else
            _adaptSequence(monad);
    };

    return {
        step: () => {
            links.forEach(_clearLink);
            monads.forEach(_advancePhase);
        },

        send: () => monads.forEach(_send),

        receive: () => monads.forEach(_receive),

        resolve: () => links.filter(_isBounced).forEach(_flip),
    };
};