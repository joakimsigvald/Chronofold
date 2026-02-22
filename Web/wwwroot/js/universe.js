import { CreateMonadLayer } from './monadLayer.js';
import { CreateLabelLayer } from './labelLayer.js';

export const CreateUniverse = (vacuum, showLabels) => {
    let svg = null;
    let view = null;
    const labelsLayer = CreateLabelLayer(vacuum);
    const monadsLayer = CreateMonadLayer(vacuum);

    const _render = () => {
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

    return {
        render() {
            _render();
            monadsLayer.render(view);
            if (showLabels)
                labelsLayer.render(view);
        },
        update() {
            monadsLayer.update();
            if (showLabels)
                labelsLayer.update();
        },
        scale(scale) {
            _scale();
            monadsLayer.scale(scale);
            if (showLabels)
                labelsLayer.scale(scale);
        },
    };
};