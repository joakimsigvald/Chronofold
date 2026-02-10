import { ChronofoldApi } from './api.js';
import { Scale } from './config.js';
import { VacuumLayer } from './vacuumLayer.js';
import { MonadLayer } from './monadLayer.js';
import { LabelLayer } from './labelLayer.js';

const Observatory = {
    vacuum: {
        monads: [],
    },
    stage: {
        scale: Scale.initialUnitSize,
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
        this.stage.scale = window.innerWidth / Scale.resolution;
        this.scaleUniverse();
    },

    renderUniverse() {
        this.stage.view = VacuumLayer.render();
        MonadLayer.render(this.stage.view, this.vacuum.monads, this.vacuum.links);
        LabelLayer.render(this.stage.view, this.vacuum.monads, this.vacuum.links);
    },

    scaleUniverse() {
        VacuumLayer.scale();
        MonadLayer.scale(this.stage.scale, this.vacuum.monads, this.vacuum.links);
        LabelLayer.scale(this.stage.scale, this.vacuum.monads, this.vacuum.links);
    },
};

Observatory.init();