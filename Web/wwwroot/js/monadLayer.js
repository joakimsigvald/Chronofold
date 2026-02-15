import { Color, Scale } from './config.js';

export const CreateMonadLayer = (vacuum) => {

    const { monads, links } = vacuum;

    const _renderCircles = (view, type, circles, getColor) =>
        view.append("g").attr("class", `${type}s-layer`)
            .selectAll("circle")
            .data(circles)
            .enter()
            .append("circle")
            .attr("class", type)
            .attr("fill", getColor);

    const _getActivePort = (monad, tick) => monad.sequence[tick];

    const _getPort = (link, monad) => monad.links.indexOf(link.id);

    const _isActiveSlot = (link, monad, tick) => _getPort(link, monad) === _getActivePort(monad, tick);

    const _isActive = (link, tick) => _isActiveSlot(link, link.left, tick) && _isActiveSlot(link, link.right, tick);

    const _getLinkColor = (link, tick) => _isActive(link, tick) ? link.color : Color.darkgrey;

    const _scaleCircles = (scale, type, circles, size) =>
        d3.select(`.${type}s-layer`).selectAll("circle")
            .data(circles)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", size * scale);

    return {
        render(view) {
            _renderCircles(view, 'monad', monads, _ => Color.lightgrey);
            _renderCircles(view, 'link', links, d => _getLinkColor(d, 0));
        },

        update(tick) {
            d3.select(".links-layer").selectAll("circle")
                .data(links)
                .transition()
                .duration(100)
                .attr("fill", d => _getLinkColor(d, tick));
        },

        scale(scale) {
            _scaleCircles(scale, 'monad', monads, Scale.monad);
            _scaleCircles(scale, 'link', links, Scale.link);
        },
    };
};