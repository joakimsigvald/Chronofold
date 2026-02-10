import { Color, Scale } from './config.js';

export const MonadLayer = {

    render(view, monads, links) {
        this._renderCircles(view, 'monad', monads, _ => Color.lightgrey);
        this._renderCircles(view, 'link', links, d => this._getLinkColor(d));
    },

    scale(scale, monads, links) {
        this._scaleCircles(scale, 'monad', monads, Scale.monad);
        this._scaleCircles(scale, 'link', links, Scale.link);
    },

    _renderCircles(view, type, circles, getColor) {
        view.append("g").attr("class", `${type}s-layer`)
            .selectAll("circle")
            .data(circles)
            .enter()
            .append("circle")
            .attr("class", type)
            .attr("fill", getColor);
    },

    _getLinkColor(link) {
        console.log(link.color);
        return link.isActive && link.color || Color.darkgrey;
    },

    _scaleCircles(scale, type, circles, size) {
        d3.select(`.${type}s-layer`).selectAll("circle")
            .data(circles)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", size * scale);
    },
};