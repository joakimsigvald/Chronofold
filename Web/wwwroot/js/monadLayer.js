import { Color, Scale } from './config.js';

export const CreateMonadLayer = (vacuum) => {
    const { monads, links, paletteSize } = vacuum;

    const _renderCircles = (view, type, circles, getColor) =>
        view.append("g").attr("class", `${type}s-layer`)
            .selectAll("circle")
            .data(circles)
            .enter()
            .append("circle")
            .attr("class", type)
            .attr("fill", getColor);

    const _getLinkColor = (link) =>
        link.isRightActive ? link.color
            : link.isLeftActive ? d3.interpolateRgb(Color.darkgrey, link.color)(0.2)
                : Color.darkgrey;

    const _getMonadColor = (monad) =>
        monad.isStressed ? Color.white : Color.lightgrey;

    const _scaleCircles = (scale, type, circles, size) =>
        d3.select(`.${type}s-layer`).selectAll("circle")
            .data(circles)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", size * scale);

    return {
        render(view) {
            _renderCircles(view, 'monad', monads, d => _getMonadColor(d));
            _renderCircles(view, 'link', links, d => _getLinkColor(d));
        },

        update() {
            d3.select(".links-layer").selectAll("circle")
                .data(links)
                .attr("fill", d => _getLinkColor(d));
            d3.select(".monads-layer").selectAll("circle")
                .data(monads)
                .attr("fill", d => _getMonadColor(d));
        },

        scale(scale) {
            _scaleCircles(scale, 'monad', monads, Scale.monad);
            _scaleCircles(scale, 'link', links, Scale.link);
        },
    };
};