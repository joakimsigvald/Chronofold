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

    const _mapLinks = (links) => {
        const monadMap = new Map(monads.map(m => [m.id, m]));
        return links.map(l => (
            {
                id: `${l.left_id}-${l.right_id}`,
                source: monadMap.get(l.left_id),
                target: monadMap.get(l.right_id),
                strength: l.strength
            }));
    }

    return {
        init() {
            _init();
            vacuum.init(view);
        },
        update(state) {
            _updateMonads(state.monads);
            vacuum.update(monads, _mapLinks(state.links));
        },
        scale(scale) {
            _scale();
            vacuum.scale(scale);
        },
    };
};