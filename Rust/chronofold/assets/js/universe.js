import { CreateMonadLayer } from './monadLayer.js';

export const CreateUniverse = (vacuum) => {
    let svg = null;
    let view = null;
    const monadsLayer = CreateMonadLayer(vacuum);

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

    return {
        init() {
            _init();
            monadsLayer.init(view);
        },
        update() {
            monadsLayer.update();
        },
        scale(scale) {
            _scale();
            monadsLayer.scale(scale);
        },
    };
};