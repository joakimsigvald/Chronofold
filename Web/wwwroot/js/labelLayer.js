import { Color, Scale } from './config.js';

export const LabelLayer = {
    render(view, monads, links) {
        const labelsLayer = view.append("g").attr("class", "labels-layer");
        this._create(labelsLayer, 'monad-label', monads, Color.black);
        this._create(labelsLayer, 'link-label', links, Color.white);
    },

    scale(scale, monads, links) {
        this._update(".monad-label", monads, scale, Scale.monadLabel);
        this._update(".link-label", links, scale, Scale.linkLabel);
    },

    _create(layer, className, data, color) {
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
            .text(d => d.id);
    },

    _update(selector, data, scale, fontSizeFactor) {
        d3.selectAll(selector)
            .data(data)
            .attr("x", d => d.x * scale)
            .attr("y", d => d.y * scale)
            .style("font-size", `${scale * fontSizeFactor}px`);
    }
};