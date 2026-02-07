const Observatory = {
    state: {
        monads: [],
        scale: 50
    },
    stage: {
        svg: null,
        g: null
    },

    async init() {
        console.log("Chronofold Observatory: Online");
        window.addEventListener("resize", () => this.onResize());
        try {
            const version = await ChronofoldApi.getVersion();
            console.log("Backend:", version);

            this.state.monads = await ChronofoldApi.getMonads();
            this.createCircles();
            this.positionCircles();

        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    onResize() {
        this.state.scale = window.innerWidth * 0.02;
        this.positionCircles();
    },

    createCircles() {
        const svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);
        svg.selectAll("*").remove();
        const view = svg.append("g");
        view.selectAll("circle")
            .data(this.state.monads)
            .enter()
            .append("circle")
            .attr("fill", "#a0a0a0")
            .attr("stroke", "#444444")
            .attr("stroke-width", 2);
    },

    positionCircles() {
        const scale = this.state.scale;
        const w = window.innerWidth;
        const h = window.innerHeight;
        const svg = d3.select("#universe")
            .attr("width", w)
            .attr("height", h);
        svg.select("g")
            .attr("transform", `translate(${w / 2}, ${h / 2})`);
        d3.select("#universe g").selectAll("circle")
            .data(this.state.monads)
            .attr("cx", d => d.x * scale)
            .attr("cy", d => d.y * scale)
            .attr("r", scale);
    }
};

// Start the lab
Observatory.init();