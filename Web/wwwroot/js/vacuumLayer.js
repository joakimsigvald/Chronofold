export const VacuumLayer = {

    svg: null,
    view: null,

    render() {
        this.svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        this.svg.selectAll("*").remove();
        this.view = this.svg.append("g");
        return this.view;
    },

    scale() {
        const w = window.innerWidth;
        const h = window.innerHeight;
        this.svg
            .attr("width", w)
            .attr("height", h);
        this.view
            .attr("transform", `translate(${w / 2}, ${h / 2})`);
    },
};