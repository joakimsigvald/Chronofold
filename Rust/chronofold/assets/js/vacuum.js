import { Color, Scale } from './config.js';

export const CreateVacuum = () => {

    let _currentScale = 1.0;
    let _nodeElements = null;
    let _linkElements = null;
    let _linkGroup = null;
    let _monadGroup = null;

    const _simulation = d3.forceSimulation()
        .force("link", d3.forceLink().id(d => d.id).distance(2).strength(0.5))
        .force("charge", d3.forceManyBody().strength(-1))
        .force("x", d3.forceX(0).strength(0.1))
        .force("y", d3.forceY(0).strength(0.1))
        .on("tick", () => {
            _linkElements
                ?.attr("x1", d => d.source.x * _currentScale)
                .attr("y1", d => d.source.y * _currentScale)
                .attr("x2", d => d.target.x * _currentScale)
                .attr("y2", d => d.target.y * _currentScale);
            _nodeElements
                ?.attr("cx", d => d.x * _currentScale)
                .attr("cy", d => d.y * _currentScale);
        });

    const _fill = (monad) => { 
        const r = monad.fugacity;
        const g = monad.affinity;
        const b = 1;
        const red = Math.round(r * 255);
        const green = Math.round(g * 255);
        const blue = Math.round(b * 255);
        return `rgb(${red}, ${green}, ${blue})`;
    };

    return {
        init(view) {
            _linkGroup = view.append("g").attr("class", "links-layer");
            _monadGroup = view.append("g").attr("class", "monads-layer");
        },

        update(monads, links) {
            _simulation.nodes(monads);
            _simulation.force("link").links(links);

            _linkElements = _linkGroup.selectAll("line.handshake")
                // Use a composite key so D3 accurately tracks specific pair connections
                .data(links, d => `${d.source.id}-${d.target.id}`)
                .join("line")
                .attr("class", "handshake")
                .attr("stroke", Color.white)
                .attr("stroke-width", Scale.linkWidth * _currentScale)
                .attr("stroke-opacity", 0.6);

            _nodeElements = _monadGroup.selectAll("circle.monad")
                .data(monads, d => d.id)
                .join("circle")
                .attr("class", "monad")
                .attr("fill", d => _fill(d))
                .attr("r", _currentScale);

            _simulation.alpha(1).restart();
        },

        scale(scale) {
            _currentScale = scale;
            _nodeElements?.attr("r", _currentScale);
            _linkElements?.attr("stroke-width", Scale.linkWidth * _currentScale);
            _simulation.alpha(0.3).restart();
        },
    };
};