import { Color, Scale } from './config.js';

export const CreateLabelLayer = (vacuum) => {

    const { monads, links } = vacuum;

    const _create = (layer, className, data, color) => {
        layer.selectAll(`text.${className}`)
            .data(data)
            .enter()
            .append("text")
            .attr("class", className)
            .attr("text-anchor", "middle")
            .attr("dominant-baseline", "central")
            .attr("fill", color)
            .style("font-family", "sans-serif")
            .style("font-weight", "bold")
            .style("pointer-events", "none")
            .text(d => d.charge);
    };

    const _update = (selector, data) => {
        d3.selectAll(selector)
            .data(data)
            .text(d => d.charge);
    };

    const _scale = (selector, data, scale, fontSizeFactor) => {
        d3.selectAll(selector)
            .data(data)
            .attr("x", d => d.x * scale)
            .attr("y", d => d.y * scale)
            .style("font-size", `${scale * fontSizeFactor}px`);
    };

    return {
        render(view) {
            const labelsLayer = view.append("g").attr("class", "labels-layer");
            _create(labelsLayer, 'monad-label', monads, Color.black);
            //_create(labelsLayer, 'link-label', links, Color.white);
        },


        update() {
            _update(".monad-label", monads);
            //_update(".link-label", links);
        },


        scale(scale) {
            _scale(".monad-label", monads, scale, Scale.monadLabel);
            //_scale(".link-label", links, scale, Scale.linkLabel);
        },
    };
};