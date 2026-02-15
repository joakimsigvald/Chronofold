import { ChronofoldApi } from './api.js';
import { Scale } from './config.js';
import { VacuumLayer } from './vacuumLayer.js';
import { CreateMonadLayer } from './monadLayer.js';
import { CreateLabelLayer } from './labelLayer.js';

const Observatory = {
    tick: 0,
    vacuum: null,
    showLabels: false,
    layers: {},

    async init() {
        console.log("Chronofold Observatory: Online");
        try {
            this.vacuum = await ChronofoldApi.getVacuum();
            this.layers.labels = CreateLabelLayer(this.vacuum);
            this.layers.monads = CreateMonadLayer(this.vacuum);
            this.render();
            this.onResize();
            window.addEventListener("resize", () => this.onResize());
            console.log("Starting Heartbeat.");
            this.startHeartbeat(300);
        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    onResize() {
        this.scale(window.innerWidth / Scale.resolution);
    },

    render() {
        const view = VacuumLayer.render();
        this.layers.monads.render(view);
        if (this.showLabels)
            this.layers.labels.render(view);
    },

    scale(scale) {
        VacuumLayer.scale();
        this.layers.monads.scale(scale);
        if (this.showLabels)
            this.layers.labels.scale(scale);
    },

    startHeartbeat(interval = 1000) {
        setInterval(() => {
            console.log("Beat...");
            this.tick = (this.tick + 1) % 6;
            if (this.layers.monads.update)
                this.layers.monads.update(this.tick);
        }, interval);
    }
};

Observatory.init();