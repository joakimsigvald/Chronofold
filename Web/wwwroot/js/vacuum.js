const Observatory = {
    vacuum: {
        monads: [],
    },
    stage: {
        scale: 50,
        svg: null,
        g: null
    },

    async init() {
        console.log("Chronofold Observatory: Online");
        window.addEventListener("resize", () => this.onResize());
        try {
            const version = await ChronofoldApi.getVersion();
            console.log("Backend:", version);

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
        const view = this.renderVacuum();
        this.renderMonads(view);
        this.renderLinks(view);
    },

    renderVacuum() {
        const svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        svg.selectAll("*").remove();
        return svg.append("g");
    },

    renderMonads(view) {
        view.append("g").attr("class", "monads-layer")
            .selectAll("circle")
            .data(this.vacuum.monads)
            .enter()
            .append("circle")
            .attr("class", "monad")
            .attr("fill", "#a0a0a0")
            .attr("stroke", "#444444")
            .attr("stroke-width", 2);
    },

    renderLinks(view) {
        view.append("g").attr("class", "links-layer")
            .selectAll("circle")
            .data(this.vacuum.links)
            .enter()
            .append("circle")
            .attr("class", "link")
            .attr("fill", "#444444"); // Darker/smaller connection point
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
        const scale = this.stage.scale;
        d3.select(".monads-layer").selectAll("circle")
            .data(this.vacuum.monads)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", scale);
    },

    scaleLinks() {
        const scale = this.stage.scale;
        d3.select(".links-layer").selectAll("circle")
            .data(this.vacuum.links)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", 0.4 * scale);
    }
};

// Start the lab
Observatory.init();