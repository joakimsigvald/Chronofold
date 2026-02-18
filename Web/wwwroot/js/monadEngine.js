export const CreateMonadEngine = (vacuum) => {
    const { monads, links, paletteSize } = vacuum;
    const linkMap = Object.fromEntries(links.map(l => [l.id, l]));

    const _getPort = (link, monad) => monad.links.indexOf(link.id);
    const _isActivePort = (link, monad) => _getPort(link, monad) === monad.activePort;
    const _isActive = (link) => _isActivePort(link, link.left) && _isActivePort(link, link.right);
    const _isStrained = (monad) => {
        const linkId = monad.links[monad.activePort];
        const link = linkMap[linkId];
        return link && !link.isActive;
    };

    return {
        updateActivePorts: (tick) => monads.forEach(monad => {
            monad.phase = (monad.initialPhase + tick) % paletteSize;
            monad.activePort = monad.sequence[monad.phase];
        }),
        updateActiveLinks: () => links.forEach(l => l.isActive = _isActive(l)),
        updateStrain: () => monads.forEach(m => m.isStrained = _isStrained(m)),
        releaveStrain: () => monads
            .filter(m => m.isStrained)
            .forEach(m => {
                const next = (m.phase + 1) % paletteSize;
                m.sequence[m.phase] = m.sequence[next];
                m.sequence[next] = m.activePort;
            })
    };
};