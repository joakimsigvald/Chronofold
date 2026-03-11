import { Color, Scale } from './config.js';

export const CreateMonadLayer = (vacuum) => {
    let currentScale = 1.0;
    let nodeElements = null;
    let viewGroup = null;

    const simulation = d3.forceSimulation()
        .force("charge", d3.forceManyBody().strength(-1))
        .force("x", d3.forceX(0).strength(0.1))
        .force("y", d3.forceY(0).strength(0.1))
        .on("tick", () => {
            if (!nodeElements) return;
            nodeElements
                .attr("cx", d => d.x * currentScale)
                .attr("cy", d => d.y * currentScale);
        });

    return {
        init(view) {
            viewGroup = view.append("g").attr("class", "monads-layer");
        },

        update() {
            simulation.nodes(vacuum.monads);
            nodeElements = viewGroup.selectAll("circle.monad")
                .data(vacuum.monads, d => d.id)
                .join("circle")
                .attr("class", "monad")
                .attr("fill", Color.white) // Simplified for the vacuum state
                .attr("r", Scale.monad * currentScale);

            simulation.alpha(1).restart();
        },

        scale(scale) {
            currentScale = scale;
            if (nodeElements) {
                nodeElements.attr("r", Scale.monad * currentScale);
            }
            simulation.alpha(0.3).restart();
        },
    };
};