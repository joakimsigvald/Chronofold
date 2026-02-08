const Observatory = {
    black: "#050505",
    darkgrey: "#444444",
    lightgrey: "#a0a0a0",
    white: "#ffffff",

    vacuum: {
        monads: [],
    },
    stage: {
        scale: 50,
        view: null,
        svg: null,
        g: null
    },

    async init() {
        console.log("Chronofold Observatory: Online");
        window.addEventListener("resize", () => this.onResize());
        try {
            this.vacuum = await ChronofoldApi.getVacuum();
            this.renderUniverse();
            this.scaleUniverse();
        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    onResize() {
        this.stage.scale = window.innerWidth * 0.02;
        this.scaleUniverse();
    },

    renderUniverse() {
        this.stage.view = this.renderVacuum();
        this.renderMonads();
        this.renderLinks();
        this.renderLabels();
    },

    renderVacuum() {
        const svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        svg.selectAll("*").remove();
        return svg.append("g");
    },

    renderMonads() {
        this.renderCircles('monad', this.vacuum.monads, this.lightgrey);
    },

    renderLinks() {
        this.renderCircles('link', this.vacuum.links, this.darkgrey);
    },

    renderCircles(type, circles, color) {
        this.stage.view.append("g").attr("class", `${type}s-layer`)
            .selectAll("circle")
            .data(circles)
            .enter()
            .append("circle")
            .attr("class", type)
            .attr("fill", color);
    },

    renderLabels() {
        this.stage.view.append("g").attr("class", "labels-layer")
            .selectAll("text")
            .data(this.vacuum.monads)
            .enter()
            .append("text")
            .attr("text-anchor", "middle")
            .attr("alignment-baseline", "middle")
            .attr("fill", this.black)
            .style("font-family", "sans-serif")
            .style("font-weight", "bold")
            .style("pointer-events", "none")
            .text(d => d.id);
    },

    scaleUniverse() {
        this.scaleVacuum();
        this.scaleMonads();
        this.scaleLinks();
        this.scaleLabels();
    },

    scaleVacuum() {
        const w = window.innerWidth;
        const h = window.innerHeight;
        const svg = d3.select("#universe")
            .attr("width", w)
            .attr("height", h);
        svg.select("g")
            .attr("transform", `translate(${w / 2}, ${h / 2})`);
    },

    scaleMonads() {
        this.scaleCircles('monad', this.vacuum.monads);
    },

    scaleLinks() {
        this.scaleCircles('link', this.vacuum.links, 0.5);
    },

    scaleCircles(type, circles, size = 1) {
        const scale = this.stage.scale;
        d3.select(`.${type}s-layer`).selectAll("circle")
            .data(circles)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", size * scale);
    },

    scaleLabels() {
        const scale = this.stage.scale;
        d3.select(".labels-layer").selectAll("text")
            .data(this.vacuum.monads)
            .attr("x", d => d.x * scale)
            .attr("y", d => d.y * scale)
            .style("font-size", `${scale * 0.5}px`); // Diameter is 2*scale, so this is ~45%
    }
};

// Start the lab
Observatory.init();