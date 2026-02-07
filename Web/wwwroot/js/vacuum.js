const Observatory = {
    config: {
        scale: 50,
        monadRadius: 20
    },

    async init() {
        console.log("Chronofold Observatory: Online");

        // Use the API service
        try {
            const version = await ChronofoldApi.getVersion();
            console.log("Backend:", version);

            const monads = await ChronofoldApi.getMonads();
            this.render(monads);
        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    render(monads) {
        const svg = d3.select("#universe")
            .attr("width", window.innerWidth)
            .attr("height", window.innerHeight);

        svg.selectAll("*").remove();

        const view = svg.append("g")
            .attr("transform", `translate(${window.innerWidth / 2}, ${window.innerHeight / 2})`);

        view.selectAll("circle")
            .data(monads)
            .enter()
            .append("circle")
            .attr("cx", d => d.x * this.config.scale)
            .attr("cy", d => d.y * this.config.scale)
            .attr("r", this.config.monadRadius)
            .attr("fill", "#d1d1d1")
            .attr("stroke", "#444444")
            .attr("stroke-width", 2);
    }
};

// Start the lab
Observatory.init();