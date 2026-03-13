import { Color, Scale } from './config.js';

export const CreateVacuum = () => {

    let currentScale = 1.0;
    let nodeElements = null;
    let linkElements = null;
    let linkGroup = null;
    let monadGroup = null;

    const simulation = d3.forceSimulation()
        .force("link", d3.forceLink().id(d => d.id).distance(2).strength(0.5))
        .force("charge", d3.forceManyBody().strength(-1))
        .force("x", d3.forceX(0).strength(0.1))
        .force("y", d3.forceY(0).strength(0.1))
        .on("tick", () => {
            linkElements
                ?.attr("x1", d => d.source.x * currentScale)
                .attr("y1", d => d.source.y * currentScale)
                .attr("x2", d => d.target.x * currentScale)
                .attr("y2", d => d.target.y * currentScale);
            nodeElements
                ?.attr("cx", d => d.x * currentScale)
                .attr("cy", d => d.y * currentScale);
        });

    return {
        init(view) {
            linkGroup = view.append("g").attr("class", "links-layer");
            monadGroup = view.append("g").attr("class", "monads-layer");
        },

        update(monads, links) {
            simulation.nodes(monads);
            simulation.force("link").links(links);

            linkElements = linkGroup.selectAll("line.handshake")
                // Use a composite key so D3 accurately tracks specific pair connections
                .data(links, d => `${d.source.id}-${d.target.id}`)
                .join("line")
                .attr("class", "handshake")
                .attr("stroke", Color.white)
                .attr("stroke-width", Scale.linkWidth * currentScale)
                .attr("stroke-opacity", 0.6);

            nodeElements = monadGroup.selectAll("circle.monad")
                .data(monads, d => d.id)
                .join("circle")
                .attr("class", "monad")
                .attr("fill", Color.orange)
                .attr("r", currentScale);

            simulation.alpha(1).restart();
        },

        scale(scale) {
            currentScale = scale;
            nodeElements?.attr("r", currentScale);
            linkElements?.attr("stroke-width", Scale.linkWidth * currentScale);
            simulation.alpha(0.3).restart();
        },
    };
};