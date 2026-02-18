import { ChronofoldApi } from './api.js';
import { Scale } from './config.js';
import { VacuumLayer } from './vacuumLayer.js';
import { CreateMonadLayer } from './monadLayer.js';
import { CreateLabelLayer } from './labelLayer.js';
import { CreateMonadEngine } from './monadEngine.js';

const Observatory = {
    tick: 0,
    vacuum: null,
    showLabels: false,
    layers: {},
    monadEngine: null,

    async init() {
        console.log("Chronofold Observatory: Online");
        try {
            this.vacuum = await ChronofoldApi.getVacuum();
            this.layers.labels = CreateLabelLayer(this.vacuum);
            this.layers.monads = CreateMonadLayer(this.vacuum);
            this.monadEngine = CreateMonadEngine(this.vacuum);
            this.render();
            this.onResize();
            window.addEventListener("resize", () => this.onResize());
            console.log("Starting Heartbeat.");
            this.startHeartbeat(1000);
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
            this.monadEngine.updateActivePorts(++this.tick);
            this.monadEngine.updateActiveLinks();
            this.monadEngine.updateStrain();
            this.layers.monads.update();
            this.monadEngine.releaveStrain();
        }, interval);
    }
};

Observatory.init();