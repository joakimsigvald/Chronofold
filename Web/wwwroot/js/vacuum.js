const Observatory = {
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
    },

    renderVacuum() {
        const svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        svg.selectAll("*").remove();
        return svg.append("g");
    },

    renderMonads() {
        this.renderCircles('monad', this.vacuum.monads, "#a0a0a0");
    },

    renderLinks() {
        this.renderCircles('link', this.vacuum.links, "#444444");
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

    scaleUniverse() {
        this.scaleVacuum();
        this.scaleMonads();
        this.scaleLinks();
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
    }
};

// Start the lab
Observatory.init();