import { CreateVacuum } from './vacuum.js';

export const CreateUniverse = () => {
    let svg = null;
    let view = null;
    const vacuum = CreateVacuum();
    let monads = [];

    const _init = () => {
        svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        svg.selectAll("*").remove();
        view = svg.append("g");
    };

    const _scale = () => {
        const w = window.innerWidth;
        const h = window.innerHeight;
        svg
            .attr("width", w)
            .attr("height", h);
        view
            .attr("transform", `translate(${w / 2}, ${h / 2})`);
    }

    const _updateMonads = (newMonads) => {
        const monadMap = new Map(monads.map(m => [m.id, m]));
        monads = newMonads.map(m => ({ ...monadMap.get(m.id), ...m }));
    }

    const _createLinks = (handshakes) => {
        const monadMap = new Map(monads.map(m => [m.id, m]));
        return handshakes.map(hs => (
            {
                source: monadMap.get(hs.source_id),
                target: monadMap.get(hs.target_id),
                strength: hs.strength
            }));
    }

    return {
        init() {
            _init();
            vacuum.init(view);
        },
        update(state) {
            _updateMonads(state.monads);
            const links = _createLinks(state.handshakes);
            vacuum.update(monads, links);
        },
        scale(scale) {
            _scale();
            vacuum.scale(scale);
        },
    };
};