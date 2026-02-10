import { ChronofoldApi } from './api.js';
import { Scale } from './config.js';
import { VacuumLayer } from './vacuumLayer.js';
import { CreateMonadLayer } from './monadLayer.js';
import { CreateLabelLayer } from './labelLayer.js';

const Observatory = {
    layers: {},

    async init() {
        console.log("Chronofold Observatory: Online");
        try {
            const vacuum = await ChronofoldApi.getVacuum();
            this.layers.labels = CreateLabelLayer(vacuum);
            this.layers.monads = CreateMonadLayer(vacuum);
            this.render();
            this.onResize();
            window.addEventListener("resize", () => this.onResize());
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
        this.layers.labels.render(view);
    },

    scale(scale) {
        VacuumLayer.scale();
        this.layers.monads.scale(scale);
        this.layers.labels.scale(scale);
    },
};

Observatory.init();