const Observatory = {
    black: "#050505",
    darkgrey: "#444444",
    lightgrey: "#a0a0a0",
    white: "#d0d0d0",

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
        this.renderCircles('monad', this.vacuum.monads, _ => this.lightgrey);
    },

    renderLinks() {
        this.renderCircles('link', this.vacuum.links, d => this.getLinkColor(d));
    },

    renderCircles(type, circles, getColor) {
        this.stage.view.append("g").attr("class", `${type}s-layer`)
            .selectAll("circle")
            .data(circles)
            .enter()
            .append("circle")
            .attr("class", type)
            .attr("fill", getColor);
    },

    getLinkColor(link) {
        console.log(link.color);
        return link.isActive && link.color || this.darkgrey;
    },

    renderLabels() {
        const labelsLayer = this.stage.view.append("g").attr("class", "labels-layer");
        this.createLabelGroups(labelsLayer, 'monad-label', this.vacuum.monads, this.black);
        this.createLabelGroups(labelsLayer, 'link-label', this.vacuum.links, this.white);
    },

    createLabelGroups(layer, className, data, color) {
        layer.selectAll(`text.${className}`)
            .data(data)
            .enter()
            .append("text")
            .attr("class", className)
            .attr("text-anchor", "middle")
            .attr("alignment-baseline", "middle")
            .attr("fill", color)
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
        this.updateLabelPositions(".monad-label", this.vacuum.monads, scale, 0.5);
        this.updateLabelPositions(".link-label", this.vacuum.links, scale, 0.33);
    },

    updateLabelPositions(selector, data, scale, fontSizeFactor) {
        d3.selectAll(selector)
            .data(data)
            .attr("x", d => d.x * scale)
            .attr("y", d => d.y * scale)
            .style("font-size", `${scale * fontSizeFactor}px`);
    }
};

// Start the lab
Observatory.init();